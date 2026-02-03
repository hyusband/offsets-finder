use crate::models::{OffsetCategory, OffsetResult, Target};
use regex::Regex;
use std::fs;

pub fn extract_hex(line: &str) -> Option<String> {
    let re = Regex::new(r"0x[0-9A-Fa-f]+").ok()?;
    re.find(line).map(|m| m.as_str().to_string())
}

pub fn scan_file(
    file_path: &str,
    targets: &[(OffsetCategory, Vec<Target>)],
) -> Result<Vec<OffsetResult>, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Error reading file: {}", e))?;

    let lines: Vec<&str> = content.lines().collect();
    let mut results = Vec::new();

    for (category, category_targets) in targets {
        for target in category_targets {
            let result = match target {
                Target::Fixed { name, hex } => OffsetResult {
                    name: name.clone(),
                    offset: Some(hex.clone()),
                    category: category.name().to_string(),
                },
                Target::Pattern { name, pattern } => {
                    let offset = find_pattern_in_lines(&lines, pattern);
                    OffsetResult {
                        name: name.clone(),
                        offset,
                        category: category.name().to_string(),
                    }
                }
                Target::Separator => continue,
            };
            results.push(result);
        }
    }

    Ok(results)
}

fn find_pattern_in_lines(lines: &[&str], pattern: &str) -> Option<String> {
    for line in lines {
        if line.contains(pattern) {
            if let Some(offset) = extract_hex(line) {
                return Some(offset);
            }
        }
    }
    None
}

pub fn batch_scan(
    file_paths: &[String],
    targets: &[(OffsetCategory, Vec<Target>)],
) -> Vec<(String, Result<Vec<OffsetResult>, String>)> {
    file_paths
        .iter()
        .map(|path| {
            let result = scan_file(path, targets);
            (path.clone(), result)
        })
        .collect()
}

pub fn compare_offsets(
    old_results: &[OffsetResult],
    new_results: &[OffsetResult],
) -> Vec<(String, Option<String>, Option<String>)> {
    let mut comparisons = Vec::new();

    for new_result in new_results {
        let old_offset = old_results
            .iter()
            .find(|r| r.name == new_result.name)
            .and_then(|r| r.offset.clone());

        if old_offset != new_result.offset {
            comparisons.push((
                new_result.name.clone(),
                old_offset,
                new_result.offset.clone(),
            ));
        }
    }

    comparisons
}
