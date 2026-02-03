use crate::models::{ExportFormat, GameVariant, OffsetResult};
use serde_json::json;
use std::collections::HashMap;
use std::fs;

pub fn export_results(
    results: &[OffsetResult],
    game: GameVariant,
    format: ExportFormat,
    output_path: &str,
) -> Result<(), String> {
    let content = match format {
        ExportFormat::Json => export_json(results, game),
        ExportFormat::CppHeader => export_cpp_header(results, game),
        ExportFormat::RustModule => export_rust_module(results, game),
        ExportFormat::PlainText => export_plain_text(results, game),
    };

    fs::write(output_path, content)
        .map_err(|e| format!("Failed to write export file: {}", e))
}

fn export_json(results: &[OffsetResult], game: GameVariant) -> String {
    let mut offsets_by_category: HashMap<String, HashMap<String, String>> = HashMap::new();

    for result in results {
        if let Some(offset) = &result.offset {
            offsets_by_category
                .entry(result.category.clone())
                .or_insert_with(HashMap::new)
                .insert(result.name.clone(), offset.clone());
        }
    }

    let output = json!({
        "game": game.name(),
        "timestamp": chrono::Local::now().to_rfc3339(),
        "offsets": offsets_by_category,
        "statistics": {
            "total": results.len(),
            "found": results.iter().filter(|r| r.offset.is_some()).count(),
            "missing": results.iter().filter(|r| r.offset.is_none()).count(),
        }
    });

    serde_json::to_string_pretty(&output).unwrap()
}

fn export_cpp_header(results: &[OffsetResult], game: GameVariant) -> String {
    let namespace = match game {
        GameVariant::FreeFire => "FreeFire",
        GameVariant::FreeFireMax => "FreeFireMax",
        GameVariant::FreeFireTela => "FreeFireTela",
    };

    let mut output = String::new();
    output.push_str("// Auto-generated offsets\n");
    output.push_str(&format!("// Game: {}\n", game.name()));
    output.push_str(&format!("// Generated: {}\n\n", chrono::Local::now()));
    output.push_str("#pragma once\n");
    output.push_str("#include <cstdint>\n\n");
    output.push_str(&format!("namespace {} {{\n", namespace));

    let mut current_category = String::new();
    for result in results {
        if result.category != current_category {
            if !current_category.is_empty() {
                output.push_str("\n");
            }
            output.push_str(&format!("    // {}\n", result.category));
            current_category = result.category.clone();
        }

        if let Some(offset) = &result.offset {
            let const_name = result.name.to_uppercase().replace(" ", "_");
            output.push_str(&format!(
                "    constexpr uintptr_t {} = {};\n",
                const_name, offset
            ));
        }
    }

    output.push_str("}\n");
    output
}

fn export_rust_module(results: &[OffsetResult], game: GameVariant) -> String {
    let module_name = match game {
        GameVariant::FreeFire => "freefire",
        GameVariant::FreeFireMax => "freefire_max",
        GameVariant::FreeFireTela => "freefire_tela",
    };

    let mut output = String::new();
    output.push_str("// Auto-generated offsets\n");
    output.push_str(&format!("// Game: {}\n", game.name()));
    output.push_str(&format!("// Generated: {}\n\n", chrono::Local::now()));
    output.push_str("#![allow(dead_code)]\n\n");
    output.push_str(&format!("pub mod {} {{\n", module_name));

    let mut current_category = String::new();
    for result in results {
        if result.category != current_category {
            if !current_category.is_empty() {
                output.push_str("\n");
            }
            output.push_str(&format!("    // {}\n", result.category));
            current_category = result.category.clone();
        }

        if let Some(offset) = &result.offset {
            let const_name = result
                .name
                .to_uppercase()
                .replace(" ", "_")
                .replace("-", "_");
            
            let hex_value = if offset.starts_with("0x") {
                offset.to_string()
            } else {
                format!("0x{}", offset)
            };
            
            output.push_str(&format!(
                "    pub const {}: usize = {};\n",
                const_name, hex_value
            ));
        }
    }

    output.push_str("}\n");
    output
}

fn export_plain_text(results: &[OffsetResult], game: GameVariant) -> String {
    let mut output = String::new();
    output.push_str(&format!("====== {} OFFSETS ======\n\n", game.name().to_uppercase()));

    let mut current_category = String::new();
    for result in results {
        if result.category != current_category {
            if !current_category.is_empty() {
                output.push_str("\n");
            }
            output.push_str(&format!("--- {} ---\n", result.category));
            current_category = result.category.clone();
        }

        match &result.offset {
            Some(offset) => output.push_str(&format!("{} = {}\n", result.name, offset)),
            None => output.push_str(&format!("{} = NOT FOUND\n", result.name)),
        }
    }

    let found = results.iter().filter(|r| r.offset.is_some()).count();
    let total = results.len();
    output.push_str(&format!("\n====== STATISTICS ======\n"));
    output.push_str(&format!("Found: {}/{}\n", found, total));

    output
}
