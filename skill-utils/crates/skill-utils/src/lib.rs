use anyhow::{Context, Result, anyhow};
use clap::ValueEnum;
use pathdiff::diff_paths;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Describes author metadata for a plugin manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

/// Represents the plugin manifest schema used by Claude Code plugins.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Author,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub keywords: Option<Vec<String>>,
    #[serde(default)]
    pub commands: Option<String>,
    #[serde(default)]
    pub agents: Option<String>,
    #[serde(default)]
    pub skills: Option<String>,
    #[serde(default)]
    pub hooks: Option<String>,
}

/// Owner metadata for the marketplace manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub name: String,
}

/// Entry describing a plugin inside the marketplace manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceEntry {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// Marketplace manifest structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marketplace {
    pub name: String,
    pub owner: Owner,
    pub plugins: Vec<MarketplaceEntry>,
}

/// Input options for creating a plugin scaffold and registering it.
#[derive(Debug, Clone)]
pub struct InitOptions {
    pub root: PathBuf,
    pub name: String,
    pub description: String,
    pub author: String,
}

/// Allowed bump levels to enforce monotonic semantic versions.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum BumpLevel {
    Patch,
    Minor,
    Major,
}

impl BumpLevel {
    /// Returns a bumped version that is strictly greater than the current one.
    pub fn bump(self, current: &Version) -> Version {
        match self {
            BumpLevel::Patch => Version::new(current.major, current.minor, current.patch + 1),
            BumpLevel::Minor => Version::new(current.major, current.minor + 1, 0),
            BumpLevel::Major => Version::new(current.major + 1, 0, 0),
        }
    }
}

/// Creates a plugin scaffold and appends it to the marketplace manifest atomically.
/// Algorithm: compute target paths, create directories, write manifest and default command, validate manifest, then update marketplace entry.
pub fn init_plugin(opts: InitOptions) -> Result<PathBuf> {
    let plugin_dir = opts.root.join("plugins").join(&opts.name);
    if plugin_dir.exists() {
        return Err(anyhow!(
            "Plugin directory already exists: {}",
            plugin_dir.display()
        ));
    }

    fs::create_dir_all(plugin_dir.join(".claude-plugin"))
        .with_context(|| "failed to create plugin metadata directory")?;
    fs::create_dir_all(plugin_dir.join("commands"))
        .with_context(|| "failed to create commands directory")?;

    let manifest = PluginManifest {
        name: opts.name.clone(),
        description: opts.description.clone(),
        version: "0.1.0".to_string(),
        author: Author {
            name: opts.author.clone(),
            email: None,
            url: None,
        },
        homepage: None,
        keywords: None,
        commands: Some("./commands/".to_string()),
        agents: None,
        skills: None,
        hooks: None,
    };

    write_manifest(&plugin_dir, &manifest)?;
    write_default_command(&plugin_dir, &manifest.name, &manifest.description)?;

    // Validate manifest structure before registering it.
    validate_plugin(&plugin_dir.join(".claude-plugin").join("plugin.json"))?;

    // Register in marketplace after creation succeeds.
    add_to_marketplace(&opts.root, &manifest, &plugin_dir)?;

    Ok(plugin_dir)
}

/// Validates a plugin manifest against required fields and file presence.
/// Algorithm: parse JSON, ensure mandatory strings are non-empty, validate semver, and confirm referenced directories exist.
pub fn validate_plugin(manifest_path: &Path) -> Result<PluginManifest> {
    let manifest_data = fs::read_to_string(manifest_path)
        .with_context(|| format!("failed to read manifest at {}", manifest_path.display()))?;
    if manifest_data.trim().is_empty() {
        return Err(anyhow!("plugin manifest is empty"));
    }

    let manifest: PluginManifest =
        serde_json::from_str(&manifest_data).with_context(|| "manifest JSON is invalid")?;

    if manifest.name.trim().is_empty() {
        return Err(anyhow!("manifest.name is required and cannot be empty"));
    }
    if manifest.description.trim().is_empty() {
        return Err(anyhow!(
            "manifest.description is required and cannot be empty"
        ));
    }

    Version::parse(&manifest.version)
        .with_context(|| "manifest.version must be valid semantic version")?;

    let plugin_root = manifest_path
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow!("could not resolve plugin directory from manifest path"))?;

    if let Some(commands) = &manifest.commands {
        let commands_path = plugin_root.join(commands);
        if !commands_path.exists() {
            return Err(anyhow!(
                "commands path referenced by manifest does not exist: {}",
                commands_path.display()
            ));
        }
    }

    Ok(manifest)
}

/// Adds or updates a plugin entry in the marketplace manifest.
/// Algorithm: load marketplace (or bootstrap default), ensure single entry per plugin, update source/description, and persist.
pub fn add_to_marketplace(root: &Path, manifest: &PluginManifest, plugin_dir: &Path) -> Result<()> {
    let marketplace_path = root.join(".claude-plugin").join("marketplace.json");
    if let Some(parent) = marketplace_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| "failed to create .claude-plugin directory for marketplace")?;
    }

    let mut marketplace = load_marketplace(&marketplace_path)?;
    let relative_source = diff_paths(plugin_dir, root).unwrap_or_else(|| plugin_dir.to_path_buf());
    let relative_source = relative_source.to_string_lossy().to_string();

    let entry = MarketplaceEntry {
        name: manifest.name.clone(),
        source: relative_source,
        description: Some(manifest.description.clone()),
    };

    if let Some(existing) = marketplace
        .plugins
        .iter_mut()
        .find(|p| p.name == manifest.name)
    {
        // Update in-place to keep a single authoritative entry.
        existing.source = entry.source;
        existing.description = entry.description;
    } else {
        marketplace.plugins.push(entry);
    }

    write_marketplace(&marketplace_path, &marketplace)
}

/// Bumps the plugin manifest version in-place with a monotonic increase.
/// Algorithm: parse manifest, parse semver, bump according to level, write back.
pub fn bump_version(manifest_path: &Path, level: BumpLevel) -> Result<Version> {
    let mut manifest = validate_plugin(manifest_path)?;
    let current = Version::parse(&manifest.version)
        .with_context(|| "manifest.version must be valid semantic version")?;
    let bumped = level.bump(&current);

    if bumped <= current {
        return Err(anyhow!(
            "bumped version {} is not greater than current {}",
            bumped,
            current
        ));
    }

    manifest.version = bumped.to_string();
    let plugin_dir = manifest_path
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow!("could not resolve plugin directory from manifest path"))?;
    write_manifest(plugin_dir, &manifest)?;

    Ok(bumped)
}

/// Writes a plugin manifest to the canonical path inside the plugin directory.
fn write_manifest(plugin_dir: &Path, manifest: &PluginManifest) -> Result<()> {
    let manifest_path = plugin_dir.join(".claude-plugin").join("plugin.json");
    let contents = serde_json::to_string_pretty(manifest)
        .with_context(|| "failed to serialize plugin manifest")?;
    let mut file = fs::File::create(&manifest_path)
        .with_context(|| format!("failed to create {}", manifest_path.display()))?;
    file.write_all(contents.as_bytes())
        .with_context(|| "failed to write plugin manifest")?;
    Ok(())
}

/// Writes a default command file so new plugins are runnable.
fn write_default_command(plugin_dir: &Path, name: &str, description: &str) -> Result<()> {
    let command_path = plugin_dir.join("commands").join(format!("{name}.md"));
    let template = format!(
        r#"---
description: {description}
tags: [generated]
---

# {name}

Describe your command behavior here. Update this file with real instructions for the plugin consumer.
"#
    );
    let mut file = fs::File::create(&command_path)
        .with_context(|| format!("failed to create {}", command_path.display()))?;
    file.write_all(template.as_bytes())
        .with_context(|| "failed to write default command file")?;
    Ok(())
}

/// Loads marketplace manifest, defaulting to a fresh scaffold when missing or empty.
fn load_marketplace(path: &Path) -> Result<Marketplace> {
    if !path.exists() {
        return Ok(default_marketplace());
    }
    let data = fs::read_to_string(path)
        .with_context(|| format!("failed to read marketplace at {}", path.display()))?;
    if data.trim().is_empty() {
        return Ok(default_marketplace());
    }
    let marketplace: Marketplace =
        serde_json::from_str(&data).with_context(|| "marketplace JSON is invalid")?;
    Ok(marketplace)
}

/// Writes marketplace manifest in pretty JSON form.
fn write_marketplace(path: &Path, marketplace: &Marketplace) -> Result<()> {
    let contents = serde_json::to_string_pretty(marketplace)
        .with_context(|| "failed to serialize marketplace")?;
    let mut file =
        fs::File::create(path).with_context(|| format!("failed to create {}", path.display()))?;
    file.write_all(contents.as_bytes())
        .with_context(|| "failed to write marketplace file")?;
    Ok(())
}

/// Returns a default marketplace scaffold aligned with the repo name.
fn default_marketplace() -> Marketplace {
    Marketplace {
        name: "marketplace-of-abeeshake".to_string(),
        owner: Owner {
            name: "abeeshake".to_string(),
        },
        plugins: Vec::new(),
    }
}

/// Runs the end-to-end pre-commit flow for plugin manifests.
/// Algorithm: decide manifest list (explicit or staged detection), validate each, bump patch version, update marketplace, optionally git-add touched files.
pub fn precommit_flow(
    root: &Path,
    manifests: Option<Vec<PathBuf>>,
    skip_stage: bool,
) -> Result<Vec<PathBuf>> {
    let manifest_list = if let Some(items) = manifests {
        items
    } else {
        detect_staged_manifests(root)?
    };

    if manifest_list.is_empty() {
        return Ok(Vec::new());
    }

    let marketplace_path = root.join(".claude-plugin").join("marketplace.json");
    let mut processed = Vec::new();

    for manifest in manifest_list {
        let manifest_abs = if manifest.is_absolute() {
            manifest.clone()
        } else {
            root.join(&manifest)
        };

        if !manifest_abs.exists() {
            return Err(anyhow!("manifest not found: {}", manifest_abs.display()));
        }

        validate_plugin(&manifest_abs)?;
        bump_version(&manifest_abs, BumpLevel::Patch)?;
        let manifest_data = validate_plugin(&manifest_abs)?;

        let plugin_dir = manifest_abs
            .parent()
            .and_then(|p| p.parent())
            .ok_or_else(|| anyhow!("could not resolve plugin directory from manifest path"))?;

        add_to_marketplace(root, &manifest_data, plugin_dir)?;

        if !skip_stage {
            git_add(&[manifest_abs.clone(), marketplace_path.clone()], root)?;
        }

        processed.push(manifest_abs);
    }

    Ok(processed)
}

/// Detects staged plugin manifests relative to the repository root.
/// Algorithm: run `git diff --cached --name-only` filtering for `plugins/*/.claude-plugin/plugin.json`, dedupe, and return paths relative to root.
fn detect_staged_manifests(root: &Path) -> Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .args([
            "diff",
            "--cached",
            "--name-only",
            "--",
            "plugins/*/.claude-plugin/plugin.json",
        ])
        .current_dir(root)
        .output()
        .with_context(|| "failed to run git diff for staged manifests")?;

    if !output.status.success() {
        return Err(anyhow!("git diff failed with status {}", output.status));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut set = HashSet::new();
    let mut results = Vec::new();
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if set.insert(trimmed.to_string()) {
            results.push(PathBuf::from(trimmed));
        }
    }
    Ok(results)
}

/// Git-adds a list of paths relative to the repository root.
/// Algorithm: convert each path to a repo-relative string, run `git add <paths>` in root, and fail if git reports error.
fn git_add(paths: &[PathBuf], root: &Path) -> Result<()> {
    let mut args: Vec<String> = Vec::new();
    for path in paths {
        let rel = diff_paths(path, root).unwrap_or_else(|| path.to_path_buf());
        args.push(rel.to_string_lossy().to_string());
    }

    let status = Command::new("git")
        .arg("add")
        .args(&args)
        .current_dir(root)
        .status()
        .with_context(|| "failed to run git add")?;

    if !status.success() {
        return Err(anyhow!("git add failed for {:?}", args));
    }

    Ok(())
}
