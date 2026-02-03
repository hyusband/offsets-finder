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
    let mut results = Vec::new();

    // Check if it's likely a binary file
    let is_binary = file_path.ends_with(".bin") || file_path.ends_with(".so") || file_path.ends_with(".dll");

    if is_binary {
        let content = fs::read(file_path).map_err(|e| format!("Error reading binary file: {}", e))?;
        for (category, category_targets) in targets {
            for target in category_targets {
                let offset = match target {
                    Target::AoB { signature, .. } => find_aob_in_bytes(&content, signature),
                    Target::Pattern { pattern, .. } => find_pattern_in_bytes(&content, pattern),
                    Target::Regex { pattern, .. } => find_regex_in_bytes(&content, pattern),
                    Target::Fixed { .. } => None,
                    Target::Separator => continue,
                };
                
                if let Some(name) = target.name() {
                    results.push(OffsetResult {
                        name: name.to_string(),
                        offset,
                        category: category.name().to_string(),
                    });
                }
            }
        }
        return Ok(results);
    }

    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Error reading file: {}", e))?;

    let lines: Vec<&str> = content.lines().collect();

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
                Target::Regex { name, pattern } => {
                    let offset = find_regex_in_lines(&lines, pattern);
                    OffsetResult {
                        name: name.clone(),
                        offset,
                        category: category.name().to_string(),
                    }
                }
                Target::AoB { name, .. } => OffsetResult {
                    name: name.clone(),
                    offset: None,
                    category: category.name().to_string(),
                },
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

fn find_regex_in_lines(lines: &[&str], pattern: &str) -> Option<String> {
    let re = Regex::new(pattern).ok()?;
    for line in lines {
        if re.is_match(line) {
            if let Some(offset) = extract_hex(line) {
                return Some(offset);
            }
        }
    }
    None
}

fn find_aob_in_bytes(bytes: &[u8], signature: &str) -> Option<String> {
    let tokens: Vec<&str> = signature.split_whitespace().collect();
    let mut pattern = Vec::new();
    let mut mask = Vec::new();

    for token in tokens {
        if token == "?" || token == "??" {
            pattern.push(0u8);
            mask.push(false);
        } else if let Ok(byte) = u8::from_str_radix(token, 16) {
            pattern.push(byte);
            mask.push(true);
        }
    }

    if pattern.is_empty() {
        return None;
    }

    for i in 0..=(bytes.len().saturating_sub(pattern.len())) {
        let mut found = true;
        for (j, (&p, &m)) in pattern.iter().zip(mask.iter()).enumerate() {
            if m && bytes[i + j] != p {
                found = false;
                break;
            }
        }
        if found {
            return Some(format!("0x{:X}", i));
        }
    }

    None
}

fn find_pattern_in_bytes(bytes: &[u8], pattern: &str) -> Option<String> {
    let p_bytes = pattern.as_bytes();
    if p_bytes.is_empty() { return None; }
    
    for i in 0..=(bytes.len().saturating_sub(p_bytes.len())) {
        if &bytes[i..i+p_bytes.len()] == p_bytes {
            return Some(format!("0x{:X}", i));
        }
    }
    None
}

fn find_regex_in_bytes(bytes: &[u8], pattern: &str) -> Option<String> {
    // We use the bytes version of regex for binary data
    let re = regex::bytes::Regex::new(pattern).ok()?;
    re.find(bytes).map(|m| format!("0x{:X}", m.start()))
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
