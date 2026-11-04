use bolina_daemon_lib::{autorotate, input_sync, wacom, wallpaper};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "bolina-daemon",
    author = "Andres Palumbo <andrespalumbo311@gmail.com>",
    version = "0.1.0",
    about = "Cloud-native hardware sensors, auto-rotation, and session helper daemon for Bolina OS"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Auto-rotation daemon reading IIO sensors and ThinkPad ACPI tablet mode
    Autorotate(AutorotateArgs),

    /// Wallpaper management, physical chassis-lock counter-rotation, and XOPP watcher
    Wallpaper(WallpaperArgs),

    /// Synchronizes input-hardware.kdl into DMS input.kdl
    InputSync(InputSyncArgs),

    /// Reset Wacom USB digitizer on resume from suspend
    WacomReset,
}

#[derive(Args)]
struct AutorotateArgs {
    /// Display output name
    #[arg(short, long, default_value = "eDP-1")]
    output: String,
}

#[derive(Args)]
struct WallpaperArgs {
    #[command(subcommand)]
    command: WallpaperCommands,
}

#[derive(Subcommand)]
enum WallpaperCommands {
    /// Apply pre-rendered wallpaper for current or given transform
    Apply {
        /// Target orientation: normal, 90, 180, 270
        transform: Option<String>,

        /// Force specific ping-pong set (a or b)
        #[arg(long)]
        set: Option<String>,

        /// Display output name
        #[arg(short, long, default_value = "eDP-1")]
        output: String,
    },

    /// Pre-render all 4 orientations (1:1 native 1920x1080 / 1080x1920) for given set
    Prerender {
        /// Target set (a or b)
        #[arg(default_value = "a")]
        set: String,
    },

    /// Watch ~/Pictures/task_wallpaper.xopp for changes and update wallpaper
    Watch {
        /// Display output name
        #[arg(short, long, default_value = "eDP-1")]
        output: String,
    },
}

#[derive(Args)]
struct InputSyncArgs {
    /// Directory containing input.kdl and input-hardware.kdl
    #[arg(short, long)]
    dir: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Autorotate(args) => {
            if let Err(e) = autorotate::run_autorotate(&args.output) {
                eprintln!("[autorotate] Fatal: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Wallpaper(args) => match args.command {
            WallpaperCommands::Apply {
                transform,
                set,
                output,
            } => {
                if let Err(e) = wallpaper::renderer::apply_wallpaper(
                    transform.as_deref(),
                    set.as_deref(),
                    &output,
                ) {
                    eprintln!("[wallpaper] Error: {}", e);
                    std::process::exit(1);
                }
            }
            WallpaperCommands::Prerender { set } => {
                if let Err(e) = wallpaper::renderer::prerender_all(&set) {
                    eprintln!("[wallpaper] Error: {}", e);
                    std::process::exit(1);
                }
            }
            WallpaperCommands::Watch { output } => {
                if let Err(e) = wallpaper::watcher::run_watcher(&output) {
                    eprintln!("[watcher] Fatal: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Commands::InputSync(args) => {
            let dir = args
                .dir
                .unwrap_or_else(input_sync::kdl_merger::default_config_dir);
            match input_sync::kdl_merger::sync_input_kdl(&dir) {
                Ok(true) => println!("[input-sync] Succeeded."),
                Ok(false) => println!("[input-sync] No sync required or files missing."),
                Err(e) => {
                    eprintln!("[input-sync] Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::WacomReset => {
            if let Err(e) = wacom::reset_wacom() {
                eprintln!("[wacom-reset] Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
