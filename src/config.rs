use crate::models::{GameVariant, OffsetCategory, Target};

pub fn get_game_config(variant: GameVariant) -> Vec<(OffsetCategory, Vec<Target>)> {
    match variant {
        GameVariant::FreeFire => get_freefire_offsets(),
        GameVariant::FreeFireMax => get_freefire_max_offsets(),
        GameVariant::FreeFireTela => get_freefire_tela_offsets(),
    }
}

fn get_freefire_offsets() -> Vec<(OffsetCategory, Vec<Target>)> {
    vec![
        (
            OffsetCategory::Core,
            vec![
                Target::new_fixed("StaticClass", "0x5C"),
                Target::new_fixed("MatchStatus", "0x3C"),
                Target::new_fixed("LocalPlayer", "0x7C"),
                Target::new_fixed("DictionaryEntities", "0x68"),
                Target::new_fixed("CurrentMatch", "0x50"),
                Target::new_aob("NetworkManager_AoB", "48 8B 05 ?? ?? ?? ?? 48 0F 44 C8"),
            ],
        ),
        (
            OffsetCategory::Player,
            vec![
                Target::new_fixed("Player_IsDead", "0x4C"),
                Target::new_regex("Player_Name", r"protected string [A-Z]{11}; // 0x"),
                Target::new_regex("Player_CurHealth", r"public float [A-Z]{11}; // 0x"),
                Target::new_regex("Player_MaxHealth", r"public float [A-Z]{11}; // 0x"),
                Target::new_pattern(
                    "Player_ShadowBase",
                    "public PlayerNetwork.HHCBNAPCKHF m_ShadowState;",
                ),
                Target::new_fixed("XPose", "0x78"),
                Target::new_regex("AvatarManager", r"protected AvatarManager [A-Z]{11};"),
                Target::new_fixed("Avatar", "0x94"),
                Target::new_fixed("Avatar_IsVisible", "0x7C"),
                Target::new_fixed("Avatar_Data", "0x10"),
                Target::new_fixed("Avatar_Data_IsTeam", "0x51"),
                Target::new_regex("Player_TeamID", r"public int [A-Z]{11}; // 0x"),
            ],
        ),
        (
            OffsetCategory::Camera,
            vec![
                Target::new_regex("FollowCamera", r"protected FollowCamera [A-Z]{11};"),
                Target::new_fixed("Camera", "0x14"),
                Target::new_pattern(
                    "AimRotation",
                    "private Quaternion <KCFEHMAIIINO>k__BackingField;",
                ),
                Target::new_pattern(
                    "MainCameraTransform",
                    "public Transform MainCameraTransform;",
                ),
            ],
        ),
        (
            OffsetCategory::Weapon,
            vec![
                Target::new_regex("Weapon", r"public [A-Z]{11} ActiveUISightingWeapon;"),
                Target::new_fixed("WeaponData", "0x58"),
                Target::new_fixed("WeaponRecoil", "0xC"),
                Target::new_regex("WeaponSpread", r"public float [A-Z]{11}; // 0x"),
                Target::new_regex("WeaponFireRate", r"public float [A-Z]{11}; // 0x"),
                Target::new_fixed("ViewMatrix", "0x98 + 0x24"),
            ],
        ),
        (
            OffsetCategory::Silent,
            vec![
                Target::new_pattern("Silent1", "private bool <LPEIEILIKGC>k__BackingField;"),
                Target::new_regex("Silent2", r"private [A-Z]{11} [A-Z]{11};"),
                Target::new_fixed("Silent3", "0x38"),
                Target::new_fixed("Silent4", "0x2C"),
            ],
        ),
        (
            OffsetCategory::Collision,
            vec![Target::new_regex(
                "HeadCollider",
                r"protected Collider [A-Z]{11};",
            )],
        ),
        (
            OffsetCategory::Attributes,
            vec![
                Target::new_regex(
                    "PlayerAttributes",
                    r"protected PlayerAttributes [A-Z]{11};",
                ),
                Target::new_fixed("NoReload", "0x91"),
                Target::new_regex("WalkSpeed", r"public float [A-Z]{11}; // 0x"),
            ],
        ),
        (
            OffsetCategory::Bot,
            vec![Target::new_pattern("isBot", "public bool IsClientBot;")],
        ),
        (
            OffsetCategory::Skeleton,
            vec![
                Target::new_regex("Head", r"protected ITransformNode [A-Z]{11};"),
                Target::new_regex("Root", r"protected ITransformNode [A-Z]{11};"),
                Target::new_regex("Spine", r"protected ITransformNode [A-Z]{11};"),
                Target::new_regex("Hip", r"protected ITransformNode [A-Z]{11};"),
                Target::new_regex("LeftHand", r"protected ITransformNode [A-Z]{11};"),
                Target::new_regex("RightHand", r"protected ITransformNode [A-Z]{11};"),
            ],
        ),
    ]
}

fn get_freefire_max_offsets() -> Vec<(OffsetCategory, Vec<Target>)> {
    let mut offsets = get_freefire_offsets();
    offsets.push((
        OffsetCategory::Core,
        vec![
            Target::new_fixed("MaxGraphicsSystem", "0x120"),
            Target::new_aob("MaxRenderer_AoB", "48 89 5C 24 ?? 48 89 74 24 ?? 57 48 83 EC 20 48 8B F1 41 8B D8"),
        ],
    ));
    offsets
}

fn get_freefire_tela_offsets() -> Vec<(OffsetCategory, Vec<Target>)> {
    let mut offsets = get_freefire_offsets();
    
    offsets.push((
        OffsetCategory::Weapon,
        vec![
            Target::new_aob("TELA_Aimbot_Assist", "F3 0F 10 05 ?? ?? ?? ?? F3 0F 58 05 ?? ?? ?? ?? F3 0F 11 05"),
            Target::new_aob("TELA_NoRecoil", "F3 0F 10 05 ?? ?? ?? ?? F3 0F 5C 05 ?? ?? ?? ?? 0F 28 C1"),
            Target::new_aob("TELA_InstantHit", "40 53 48 83 EC 20 48 8B DA 0F 29 74 24 ?? 48 8B 0D ?? ?? ?? ??"),
            Target::new_aob("TELA_WeaponSpread", "F3 0F 10 41 ?? F3 0F 58 41 ?? F3 0F 11 41 ?? C3"),
        ],
    ));

    offsets.push((
        OffsetCategory::Camera,
        vec![
            Target::new_aob("TELA_Wallhack_Chams", "48 8B 05 ?? ?? ?? ?? 48 8B 88 ?? ?? ?? ?? 80 BB ?? ?? ?? ?? ?? 74 ??"),
            Target::new_aob("TELA_ESP_Line", "E8 ?? ?? ?? ?? 48 8B 4B ?? 48 8B 01 FF 90 ?? ?? ?? ?? 48 8D 4D ??"),
        ],
    ));

    offsets.push((
        OffsetCategory::Attributes,
        vec![
            Target::new_aob("TELA_SpeedHack", "F3 0F 10 ?? ?? ?? ?? ?? F3 0F 59 ?? F3 0F 11 ?? ?? ?? ?? ??"),
            Target::new_aob("TELA_SkyFly", "48 8B 05 ?? ?? ?? ?? 48 8B 88 ?? ?? ?? ?? F3 0F 10 81 ?? ?? ?? ??"),
            Target::new_aob("TELA_Underground", "F3 0F 10 81 ?? ?? ?? ?? F3 0F 5C 81 ?? ?? ?? ?? F3 0F 11 81 ?? ?? ?? ??"),
        ],
    ));

    offsets.push((
        OffsetCategory::Core,
        vec![
            Target::new_fixed("TelaSystem", "0xA0"),
            Target::new_aob("TelaLocalPlayer", "48 8B 05 ?? ?? ?? ?? 48 8B 88 ?? ?? ?? ?? 48 85 C9 74 ?? 48 8B 01"),
            Target::new_aob("TelaViewMatrix", "E8 ?? ?? ?? ?? 48 8D 4C 24 ?? E8 ?? ?? ?? ?? 48 8B ?? ?? ?? ?? ?? 48 8B"),
            Target::new_aob("TelaMatchManager", "48 8B 0D ?? ?? ?? ?? 48 8B 01 48 8B 80 ?? 00 00 00 FF D0 48 8B C8"),
        ],
    ));
    
    offsets
}

pub fn detect_game_variant(content: &str) -> Option<GameVariant> {
    if content.contains("FreeFireMAX") || content.contains("MaxGraphics") {
        return Some(GameVariant::FreeFireMax);
    }

    if content.contains("FreeFireTELA") || content.contains("TelaVersion") {
        return Some(GameVariant::FreeFireTela);
    }

    if content.contains("FreeFire") || content.contains("PlayerNetwork") {
        return Some(GameVariant::FreeFire);
    }

    None
}
