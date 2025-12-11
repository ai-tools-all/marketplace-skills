use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use skill_utils::{
    add_to_marketplace, bump_version, init_plugin, validate_plugin, BumpLevel, InitOptions,
};

/// CLI entrypoint for skill-utils workspace actions.
#[derive(Parser)]
#[command(
    name = "skill-utils",
    version,
    about = "Utilities for managing Claude Code plugins in this repo"
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
    }

    Ok(())
}
