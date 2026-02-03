use crate::models::{GameVariant, OffsetResult};
use colored::*;
use std::io::{self, Write};

pub fn print_header() {
    println!("{}", "╔═══════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║     Featured OFFSETS FINDER - RUST EDITION      ║".bright_cyan().bold());
    println!("{}", "║              v1.0 - Modular Edition               ║".bright_cyan());
    println!("{}", "╚═══════════════════════════════════════════════════╝".bright_cyan());
    println!();
}

pub fn print_game_menu() {
    println!("{}", "Select Free Fire variant:".bright_blue().bold());
    println!("  {} Free Fire (Standard)", "1.".bright_yellow());
    println!("  {} Free Fire MAX", "2.".bright_yellow());
    println!("  {} Free Fire TELA", "3.".bright_yellow());
    println!("  {} Auto-detect from dump", "4.".bright_yellow());
    println!();
}

pub fn get_game_selection() -> Option<usize> {
    print!("{}", "Enter your choice (1-4): ".bright_blue());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    
    input.trim().parse::<usize>().ok()
}

pub fn print_results(results: &[OffsetResult], game: GameVariant) {
    println!("\n{}", format!("====== {} OFFSETS ======", game.name().to_uppercase()).bright_cyan().bold());
    
    let mut current_category = String::new();
    
    for result in results {
        if result.category != current_category {
            println!("\n{}", format!("--- {} ---", result.category).bright_magenta().bold());
            current_category = result.category.clone();
        }
        
        match &result.offset {
            Some(offset) => {
                println!("{} {}", 
                    result.name.bright_green().bold(), 
                    offset.bright_yellow()
                );
            }
            None => {
                println!("{} {}", 
                    result.name.bright_red().bold(), 
                    "NOT FOUND".red()
                );
            }
        }
    }
}

pub fn print_statistics(results: &[OffsetResult]) {
    let found_count = results.iter().filter(|r| r.offset.is_some()).count();
    let total_count = results.len();
    let missing_count = total_count - found_count;
    
    println!("\n{}", "═══════════════════════════════════════════".bright_cyan());
    println!("{}", "STATISTICS".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════════".bright_cyan());
    println!("{} {}", 
        "Total offsets:".bright_white(), 
        total_count.to_string().bright_yellow().bold()
    );
    println!("{} {}", 
        "Found:".bright_green().bold(),
        found_count.to_string().bright_green().bold()
    );
    println!("{} {}", 
        "Missing:".bright_red().bold(),
        missing_count.to_string().bright_red().bold()
    );
    
    let percentage = (found_count as f32 / total_count as f32) * 100.0;
    println!("{} {:.1}%", 
        "Success rate:".bright_white(),
        percentage.to_string().bright_yellow().bold()
    );
    println!("{}", "═══════════════════════════════════════════".bright_cyan());
}

pub fn print_export_menu() {
    println!("\n{}", "Export options:".bright_blue().bold());
    println!("  {} JSON format", "1.".bright_yellow());
    println!("  {} C++ header (.hpp)", "2.".bright_yellow());
    println!("  {} Rust module (.rs)", "3.".bright_yellow());
    println!("  {} Plain text (.txt)", "4.".bright_yellow());
    println!("  {} Skip export", "5.".bright_yellow());
    println!();
}

pub fn get_export_selection() -> Option<usize> {
    print!("{}", "Enter your choice (1-5): ".bright_blue());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    
    input.trim().parse::<usize>().ok()
}

pub fn print_success(message: &str) {
    println!("{} {}", "✓".bright_green().bold(), message.bright_white());
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".bright_red().bold(), message.bright_red());
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".bright_blue().bold(), message.bright_white());
}

pub fn wait_for_enter() {
    print!("\n{}", "Press Enter to exit...".bright_blue());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
