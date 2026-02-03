use colored::*;
use regex::Regex;
use rfd::FileDialog;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Clone)]
enum Target {
    Fixed(String, String),
    Pattern(String, String),
    Separator,
}

impl Target {
    fn new_fixed(name: &str, hex: &str) -> Self {
        Target::Fixed(name.to_string(), hex.to_string())
    }

    fn new_pattern(name: &str, pattern: &str) -> Self {
        Target::Pattern(name.to_string(), pattern.to_string())
    }
}

fn get_targets() -> Vec<Target> {
    vec![
        Target::new_fixed("StaticClass", "0x5C"),
        Target::new_fixed("MatchStatus", "0x3C"),
        Target::new_fixed("LocalPlayer", "0x7C"),
        Target::new_fixed("DictionaryEntities", "0x68"),
        Target::Separator,
        
        Target::new_fixed("Player_IsDead", "0x4C"),
        Target::new_pattern("Player_Name", "protected string OIAJCBLDHKP;"),
        Target::new_pattern("Player_ShadowBase", "public PlayerNetwork.HHCBNAPCKHF m_ShadowState;"),
        Target::new_fixed("XPose", "0x78"),
        Target::new_pattern("AvatarManager", "protected AvatarManager FOGJNGDMJKJ;"),
        Target::new_fixed("Avatar", "0x94"),
        Target::new_fixed("Avatar_IsVisible", "0x7C"),
        Target::new_fixed("Avatar_Data", "0x10"),
        Target::new_fixed("Avatar_Data_IsTeam", "0x51"),
        Target::new_fixed("CurrentMatch", "0x50"),
        Target::Separator,
        
        Target::new_pattern("FollowCamera", "protected FollowCamera CHDOHNOEBML;"),
        Target::new_fixed("Camera", "0x14"),
        Target::new_pattern("AimRotation", "private Quaternion <KCFEHMAIIINO>k__BackingField;"),
        Target::new_pattern("MainCameraTransform", "public Transform MainCameraTransform;"),
        Target::Separator,
        
        Target::new_pattern("Weapon", "public GPBDEDFKJNA ActiveUISightingWeapon;"),
        Target::new_fixed("WeaponData", "0x58"),
        Target::new_fixed("WeaponRecoil", "0xC"),
        Target::new_fixed("ViewMatrix", "0x98 + 0x24"),
        Target::Separator,
        
        Target::new_pattern("Silent1", "private bool <LPEIEILIKGC>k__BackingField;"),
        Target::new_pattern("Silent2", "private MADMMIICBNN GEGFCFDGGGP;"),
        Target::new_fixed("Silent3", "0x38"),
        Target::new_fixed("Silent4", "0x2C"),
        Target::Separator,
        
        Target::new_pattern("HeadCollider", "protected Collider HECFNHJKOMN;"),
        Target::Separator,
        
        Target::new_pattern("PlayerAttributes", "protected PlayerAttributes JKPFFNEMJIF;"),
        Target::new_fixed("NoReload", "0x91"),
        Target::Separator,
        
        Target::new_pattern("isBot", "public bool IsClientBot;"),
        Target::Separator,
        
        Target::new_pattern("Head", "protected ITransformNode OLCJOGDHJJJ;"),
        Target::new_pattern("Root", "protected ITransformNode MPJBGDJJJMJ;"),
        Target::new_pattern("LeftWrist", "protected ITransformNode GCMICMFEAKI;"),
        Target::new_pattern("Spine", "protected ITransformNode HCLMADAFLPD;"),
        Target::new_pattern("Hip", "protected ITransformNode CENAIGAFGAG;"),
        Target::new_pattern("RightCalf", "protected ITransformNode JPBJIMCDBHN;"),
        Target::new_pattern("LeftCalf", "protected ITransformNode BMGCHFGEDDA;"),
        Target::new_pattern("RightFoot", "protected ITransformNode AGHJLIMNPJA;"),
        Target::new_pattern("LeftFoot", "protected ITransformNode FDMBKCKMODA;"),
        Target::new_pattern("RightWrist", "protected ITransformNode CKABHDJDMAP;"),
        Target::new_pattern("LeftHand", "protected ITransformNode KOCDBPLKMBI;"),
        Target::new_pattern("LeftShoulder", "protected ITransformNode LIBEIIIAGIK;"),
        Target::new_pattern("RightShoulder", "protected ITransformNode HDEPJIBNIIK;"),
        Target::new_pattern("RightWristJoint", "protected ITransformNode NJDDAPKPILB;"),
        Target::new_pattern("LeftWristJoint", "protected ITransformNode JHIBMHEMJOL;"),
        Target::new_pattern("LeftElbow", "protected ITransformNode JBACCHNMGNJ;"),
        Target::new_pattern("RightElbow", "protected ITransformNode FGECMMJKFNC;"),
    ]
}

fn extract_hex(line: &str) -> Option<String> {
    let re = Regex::new(r"0x[0-9A-Fa-f]+").ok()?;
    re.find(line).map(|m| m.as_str().to_string())
}

fn search_offsets(file_path: &str, targets: &[Target]) -> Vec<(String, Option<String>)> {
    let mut results = Vec::new();
    
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", format!("Error reading file: {}", e).red().bold());
            return results;
        }
    };
    
    let lines: Vec<&str> = content.lines().collect();
    
    for target in targets {
        match target {
            Target::Fixed(name, hex) => {
                results.push((name.clone(), Some(hex.clone())));
            }
            Target::Pattern(name, pattern) => {
                let mut found = None;
                for line in &lines {
                    if line.contains(pattern) {
                        if let Some(offset) = extract_hex(line) {
                            found = Some(offset);
                            break;
                        }
                    }
                }
                results.push((name.clone(), found));
            }
            Target::Separator => {
                results.push((String::new(), None));
            }
        }
    }
    
    results
}

fn print_results(results: &[(String, Option<String>)]) {
    println!("\n{}", "====== OFFSETS ======".bright_cyan().bold());
    
    for (name, offset) in results {
        if name.is_empty() {
            println!();
            continue;
        }
        
        match offset {
            Some(hex) => {
                println!("{} {}", 
                    name.bright_green().bold(), 
                    hex.bright_yellow()
                );
            }
            None => {
                println!("{} {}", 
                    name.bright_red().bold(), 
                    "NOT FOUND".red()
                );
            }
        }
    }
}

fn set_console_title(_title: &str) {
    // Console title functionality removed to avoid Windows API complexity
}

fn main() {
    set_console_title("FEATURED OFFSETS FINDER - RUST EDITION");
    
    println!("{}", "╔═══════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║   OFFSETS FINDER - RUST EDITION v1.0    ║".bright_cyan().bold());
    println!("{}", "╚═══════════════════════════════════════════╝".bright_cyan());
    println!();
    
    let file_path = FileDialog::new()
        .add_filter("C# Dump", &["cs"])
        .set_title("Select dump.cs file")
        .pick_file();
    
    let file_path = match file_path {
        Some(path) => path,
        None => {
            eprintln!("{}", "No file selected. Exiting...".red().bold());
            wait_for_enter();
            return;
        }
    };
    
    println!("{} {}", 
        "Selected file:".bright_blue().bold(), 
        file_path.display().to_string().bright_white()
    );
    
    let targets = get_targets();
    println!("{}", "\nSearching for offsets...".bright_blue().bold());
    
    let results = search_offsets(file_path.to_str().unwrap(), &targets);
    
    print_results(&results);
    
    let found_count = results.iter()
        .filter(|(name, offset)| !name.is_empty() && offset.is_some())
        .count();
    let total_count = results.iter()
        .filter(|(name, _)| !name.is_empty())
        .count();
    
    println!("\n{}", "═══════════════════════════════════════════".bright_cyan());
    println!("{} {}/{}", 
        "Found:".bright_green().bold(),
        found_count.to_string().bright_yellow().bold(),
        total_count.to_string().bright_white()
    );
    println!("{}", "═══════════════════════════════════════════".bright_cyan());
    
    wait_for_enter();
}

fn wait_for_enter() {
    print!("\n{}", "Press Enter to exit...".bright_blue());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
