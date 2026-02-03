mod config;
mod dumper;
mod exporter;
mod models;
mod scanner;
mod ui;

use clap::{Parser, Subcommand};
use config::{detect_game_variant, get_game_config};
use dumper::dump_bluestacks_memory;
use exporter::export_results;
use models::{ExportFormat, GameVariant};
use rfd::FileDialog;
use scanner::scan_file;
use std::path::PathBuf;
use ui::{
    get_export_selection, get_game_selection, print_error, print_export_menu, print_game_menu,
    print_header, print_info, print_results, print_statistics, print_success, wait_for_enter,
};

#[derive(Parser)]
#[command(name = "Free Fire Offsets Finder")]
#[command(about = "Extract memory offsets from Free Fire dump files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Interactive,
    Scan {
        #[arg(short, long)]
        file: PathBuf,
        
        #[arg(short, long)]
        game: Option<String>,
        
        #[arg(short, long)]
        export: Option<String>,
        
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Dump {
        #[arg(short, long, default_value = "memory_dump.bin")]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Interactive) | None => run_interactive_mode(),
        Some(Commands::Scan {
            file,
            game,
            export,
            output,
        }) => run_cli_mode(file, game, export, output),
        Some(Commands::Dump { output }) => {
            print_header();
            print_info(&format!("Attempting to dump BlueStacks memory to {}...", output));
            match dump_bluestacks_memory(&output) {
                Ok(_) => print_success(&format!("Memory dumped successfully to {}", output)),
                Err(e) => print_error(&format!("Dump failed: {}", e)),
            }
            wait_for_enter();
        }
    }
}

fn run_interactive_mode() {
    print_header();

    print_info("Select a dump.cs file...");
    let file_path = match FileDialog::new()
        .add_filter("C# Dump", &["cs"])
        .set_title("Select dump.cs file")
        .pick_file()
    {
        Some(path) => path,
        None => {
            print_error("No file selected. Exiting...");
            wait_for_enter();
            return;
        }
    };

    print_success(&format!("Selected: {}", file_path.display()));

    print_game_menu();
    let game_variant = match get_game_selection() {
        Some(1) => GameVariant::FreeFire,
        Some(2) => GameVariant::FreeFireMax,
        Some(3) => GameVariant::FreeFireTela,
        Some(4) => {
            print_info("Auto-detecting game variant...");
            match std::fs::read_to_string(&file_path) {
                Ok(content) => match detect_game_variant(&content) {
                    Some(variant) => {
                        print_success(&format!("Detected: {}", variant.name()));
                        variant
                    }
                    None => {
                        print_error("Could not detect game variant. Using Free Fire standard.");
                        GameVariant::FreeFire
                    }
                },
                Err(_) => {
                    print_error("Error reading file. Using Free Fire standard.");
                    GameVariant::FreeFire
                }
            }
        }
        _ => {
            print_error("Invalid selection. Using Free Fire standard.");
            GameVariant::FreeFire
        }
    };

    print_info(&format!("Scanning for {} offsets...", game_variant.name()));

    let config = get_game_config(game_variant);
    let results = match scan_file(file_path.to_str().unwrap(), &config) {
        Ok(results) => results,
        Err(e) => {
            print_error(&format!("Scan failed: {}", e));
            wait_for_enter();
            return;
        }
    };

    print_results(&results, game_variant);
    print_statistics(&results);

    print_export_menu();
    if let Some(choice) = get_export_selection() {
        let (format, extension) = match choice {
            1 => (ExportFormat::Json, "json"),
            2 => (ExportFormat::CppHeader, "hpp"),
            3 => (ExportFormat::RustModule, "rs"),
            4 => (ExportFormat::PlainText, "txt"),
            5 => {
                print_info("Skipping export.");
                wait_for_enter();
                return;
            }
            _ => {
                print_error("Invalid selection. Skipping export.");
                wait_for_enter();
                return;
            }
        };

        let output_path = FileDialog::new()
            .set_file_name(&format!("offsets.{}", extension))
            .add_filter("Export file", &[extension])
            .save_file();

        if let Some(path) = output_path {
            match export_results(&results, game_variant, format, path.to_str().unwrap()) {
                Ok(_) => print_success(&format!("Exported to: {}", path.display())),
                Err(e) => print_error(&format!("Export failed: {}", e)),
            }
        } else {
            print_info("Export cancelled.");
        }
    }

    wait_for_enter();
}

fn run_cli_mode(
    file: PathBuf,
    game: Option<String>,
    export_format: Option<String>,
    output: Option<PathBuf>,
) {
    let game_variant = match game.as_deref() {
        Some("freefire") | Some("ff") => GameVariant::FreeFire,
        Some("max") => GameVariant::FreeFireMax,
        Some("tela") => GameVariant::FreeFireTela,
        Some("auto") | None => {
            match std::fs::read_to_string(&file) {
                Ok(content) => detect_game_variant(&content).unwrap_or(GameVariant::FreeFire),
                Err(_) => GameVariant::FreeFire,
            }
        }
        _ => {
            eprintln!("Invalid game variant. Using Free Fire standard.");
            GameVariant::FreeFire
        }
    };

    println!("Scanning {} for {} offsets...", file.display(), game_variant.name());

    let config = get_game_config(game_variant);
    let results = match scan_file(file.to_str().unwrap(), &config) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    print_results(&results, game_variant);
    print_statistics(&results);

    if let Some(format_str) = export_format {
        let format = match format_str.as_str() {
            "json" => ExportFormat::Json,
            "cpp" => ExportFormat::CppHeader,
            "rust" => ExportFormat::RustModule,
            "txt" => ExportFormat::PlainText,
            _ => {
                eprintln!("Invalid export format. Skipping export.");
                return;
            }
        };

        let output_path = output.unwrap_or_else(|| {
            PathBuf::from(format!("offsets.{}", format.extension()))
        });

        match export_results(&results, game_variant, format, output_path.to_str().unwrap()) {
            Ok(_) => println!("Exported to: {}", output_path.display()),
            Err(e) => eprintln!("Export failed: {}", e),
        }
    }
}
