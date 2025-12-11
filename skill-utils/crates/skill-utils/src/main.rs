use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use skill_utils::{
    BumpLevel, InitOptions, add_to_marketplace, bump_version, init_plugin, precommit_flow,
    validate_plugin,
};

#[derive(Parser)]
#[command(
    name = "skill-utils",
    version,
    about = "Utilities for managing Claude Code plugins in this repo",
    long_about = "Utilities for managing Claude Code plugins in this repo.\n\nWorkflow: run init first, then pre-commit to verify. If something fails, run steps individually. Once everything is clean, bump version.\n\nPlugin workflow (local):\n- Create: init --name <n> --description \"...\" --author \"...\"\n- Validate: validate --manifest plugins/<n>/.claude-plugin/plugin.json\n- Register: add-to-marketplace --manifest plugins/<n>/.claude-plugin/plugin.json\n- Pre-commit flow: pre-commit (auto detect staged manifests) or pre-commit --manifest <path> [--skip-stage]",
    help_template = "{name} v{version}\n{about}\n\n{long-about}\n\n{usage-heading} {usage}\n\n{all-args}\n"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Supported subcommands for plugin lifecycle management.
#[derive(Subcommand)]
enum Commands {
    /// Create a plugin scaffold and register it in the marketplace.
    Init {
        #[arg(long)]
        name: String,
        #[arg(long)]
        description: String,
        #[arg(long)]
        author: String,
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
    /// Validate a plugin manifest for required fields and structure.
    Validate {
        #[arg(long)]
        manifest: PathBuf,
    },
    /// Add or update a plugin entry in the marketplace from a manifest path.
    AddToMarketplace {
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
    /// Bump a plugin manifest version (patch/minor/major) while enforcing monotonic increase.
    BumpVersion {
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long, value_enum, default_value = "patch")]
        level: BumpLevel,
    },
    /// Run validate -> bump -> marketplace update for staged or provided manifests.
    PreCommit {
        #[arg(long)]
        manifest: Vec<PathBuf>,
        #[arg(long, default_value = ".")]
        root: PathBuf,
        #[arg(long)]
        skip_stage: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            name,
            description,
            author,
            root,
        } => {
            let opts = InitOptions {
                root,
                name,
                description,
                author,
            };
            let plugin_dir = init_plugin(opts)?;
            println!("Plugin created at {}", plugin_dir.display());
        }
        Commands::Validate { manifest } => {
            // Validate manifest structure and required fields.
            let validated = validate_plugin(&manifest)?;
            println!("Manifest is valid for plugin '{}'", validated.name);
        }
        Commands::AddToMarketplace { manifest, root } => {
            let manifest_data = validate_plugin(&manifest)?;
            let plugin_dir = manifest.parent().and_then(|p| p.parent()).ok_or_else(|| {
                anyhow::anyhow!("could not resolve plugin directory from manifest")
            })?;
            // Ensure marketplace is updated with this plugin entry.
            add_to_marketplace(&root, &manifest_data, plugin_dir)?;
            println!("Marketplace updated with plugin '{}'", manifest_data.name);
        }
        Commands::BumpVersion { manifest, level } => {
            // Auto-bump version with the requested level.
            let bumped = bump_version(&manifest, level)?;
            println!("Version bumped to {}", bumped);
        }
        Commands::PreCommit {
            manifest,
            root,
            skip_stage,
        } => {
            let processed = precommit_flow(
                &root,
                if manifest.is_empty() {
                    None
                } else {
                    Some(manifest)
                },
                skip_stage,
            )?;
            if processed.is_empty() {
                println!("No staged plugin manifests detected.");
            } else {
                println!(
                    "Processed manifests:\n{}",
                    processed
                        .iter()
                        .map(|p| format!("- {}", p.display()))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
        }
    }

    Ok(())
}
