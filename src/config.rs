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
            ],
        ),
        (
            OffsetCategory::Player,
            vec![
                Target::new_fixed("Player_IsDead", "0x4C"),
                Target::new_pattern("Player_Name", "protected string OIAJCBLDHKP;"),
                Target::new_pattern(
                    "Player_ShadowBase",
                    "public PlayerNetwork.HHCBNAPCKHF m_ShadowState;",
                ),
                Target::new_fixed("XPose", "0x78"),
                Target::new_pattern("AvatarManager", "protected AvatarManager FOGJNGDMJKJ;"),
                Target::new_fixed("Avatar", "0x94"),
                Target::new_fixed("Avatar_IsVisible", "0x7C"),
                Target::new_fixed("Avatar_Data", "0x10"),
                Target::new_fixed("Avatar_Data_IsTeam", "0x51"),
            ],
        ),
        (
            OffsetCategory::Camera,
            vec![
                Target::new_pattern("FollowCamera", "protected FollowCamera CHDOHNOEBML;"),
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
                Target::new_pattern("Weapon", "public GPBDEDFKJNA ActiveUISightingWeapon;"),
                Target::new_fixed("WeaponData", "0x58"),
                Target::new_fixed("WeaponRecoil", "0xC"),
                Target::new_fixed("ViewMatrix", "0x98 + 0x24"),
            ],
        ),
        (
            OffsetCategory::Silent,
            vec![
                Target::new_pattern("Silent1", "private bool <LPEIEILIKGC>k__BackingField;"),
                Target::new_pattern("Silent2", "private MADMMIICBNN GEGFCFDGGGP;"),
                Target::new_fixed("Silent3", "0x38"),
                Target::new_fixed("Silent4", "0x2C"),
            ],
        ),
        (
            OffsetCategory::Collision,
            vec![Target::new_pattern(
                "HeadCollider",
                "protected Collider HECFNHJKOMN;",
            )],
        ),
        (
            OffsetCategory::Attributes,
            vec![
                Target::new_pattern(
                    "PlayerAttributes",
                    "protected PlayerAttributes JKPFFNEMJIF;",
                ),
                Target::new_fixed("NoReload", "0x91"),
            ],
        ),
        (
            OffsetCategory::Bot,
            vec![Target::new_pattern("isBot", "public bool IsClientBot;")],
        ),
        (
            OffsetCategory::Skeleton,
            vec![
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
            ],
        ),
    ]
}

fn get_freefire_max_offsets() -> Vec<(OffsetCategory, Vec<Target>)> {
    // TODO: Add Free Fire MAX specific offsets
    // For now, using same as standard with note
    vec![
        (
            OffsetCategory::Core,
            vec![
                Target::new_fixed("StaticClass", "0x5C"),
                Target::new_fixed("MatchStatus", "0x3C"),
                Target::new_fixed("LocalPlayer", "0x7C"),
                Target::new_fixed("DictionaryEntities", "0x68"),
            ],
        ),
        // Add more MAX-specific offsets here
    ]
}

fn get_freefire_tela_offsets() -> Vec<(OffsetCategory, Vec<Target>)> {
    // TODO: Add Free Fire TELA specific offsets
    vec![
        (
            OffsetCategory::Core,
            vec![
                Target::new_fixed("StaticClass", "0x5C"),
                Target::new_fixed("MatchStatus", "0x3C"),
                Target::new_fixed("LocalPlayer", "0x7C"),
            ],
        ),
        // Add more TELA-specific offsets here
    ]
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
