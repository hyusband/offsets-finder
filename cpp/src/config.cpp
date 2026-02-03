#include "config.hpp"
#include <algorithm>

namespace OffsetsFinder {

GameConfig GetGameConfig(GameVariant variant) {
    switch (variant) {
        case GameVariant::FreeFire: return GetFreeFireOffsets();
        case GameVariant::FreeFireMax: return GetFreeFireMaxOffsets();
        case GameVariant::FreeFireTela: return GetFreeFireTelaOffsets();
        default: return GetFreeFireOffsets();
    }
}

GameConfig GetFreeFireOffsets() {
    return {
        {
            OffsetCategory::Core,
            {
                Target::CreateFixed("StaticClass", "0x5C"),
                Target::CreateFixed("MatchStatus", "0x3C"),
                Target::CreateFixed("LocalPlayer", "0x7C"),
                Target::CreateFixed("DictionaryEntities", "0x68"),
                Target::CreateFixed("CurrentMatch", "0x50"),
            }
        },
        {
            OffsetCategory::Player,
            {
                Target::CreateFixed("Player_IsDead", "0x4C"),
                Target::CreatePattern("Player_Name", "protected string OIAJCBLDHKP;"),
                Target::CreatePattern("Player_ShadowBase", "public PlayerNetwork.HHCBNAPCKHF m_ShadowState;"),
                Target::CreateFixed("XPose", "0x78"),
                Target::CreatePattern("AvatarManager", "protected AvatarManager FOGJNGDMJKJ;"),
                Target::CreateFixed("Avatar", "0x94"),
                Target::CreateFixed("Avatar_IsVisible", "0x7C"),
                Target::CreateFixed("Avatar_Data", "0x10"),
                Target::CreateFixed("Avatar_Data_IsTeam", "0x51"),
            }
        },
        {
            OffsetCategory::Camera,
            {
                Target::CreatePattern("FollowCamera", "protected FollowCamera CHDOHNOEBML;"),
                Target::CreateFixed("Camera", "0x14"),
                Target::CreatePattern("AimRotation", "private Quaternion <KCFEHMAIIINO>k__BackingField;"),
                Target::CreatePattern("MainCameraTransform", "public Transform MainCameraTransform;"),
            }
        },
        {
            OffsetCategory::Weapon,
            {
                Target::CreatePattern("Weapon", "public GPBDEDFKJNA ActiveUISightingWeapon;"),
                Target::CreateFixed("WeaponData", "0x58"),
                Target::CreateFixed("WeaponRecoil", "0xC"),
                Target::CreateFixed("ViewMatrix", "0x98 + 0x24"),
            }
        },
        {
            OffsetCategory::Silent,
            {
                Target::CreatePattern("Silent1", "private bool <LPEIEILIKGC>k__BackingField;"),
                Target::CreatePattern("Silent2", "private MADMMIICBNN GEGFCFDGGGP;"),
                Target::CreateFixed("Silent3", "0x38"),
                Target::CreateFixed("Silent4", "0x2C"),
            }
        },
        {
            OffsetCategory::Collision,
            {
                Target::CreatePattern("HeadCollider", "protected Collider HECFNHJKOMN;"),
            }
        },
        {
            OffsetCategory::Attributes,
            {
                Target::CreatePattern("PlayerAttributes", "protected PlayerAttributes JKPFFNEMJIF;"),
                Target::CreateFixed("NoReload", "0x91"),
            }
        },
        {
            OffsetCategory::Bot,
            {
                Target::CreatePattern("isBot", "public bool IsClientBot;"),
            }
        },
        {
            OffsetCategory::Skeleton,
            {
                Target::CreatePattern("Head", "protected ITransformNode OLCJOGDHJJJ;"),
                Target::CreatePattern("Root", "protected ITransformNode MPJBGDJJJMJ;"),
                Target::CreatePattern("LeftWrist", "protected ITransformNode GCMICMFEAKI;"),
                Target::CreatePattern("Spine", "protected ITransformNode HCLMADAFLPD;"),
                Target::CreatePattern("Hip", "protected ITransformNode CENAIGAFGAG;"),
                Target::CreatePattern("RightCalf", "protected ITransformNode JPBJIMCDBHN;"),
                Target::CreatePattern("LeftCalf", "protected ITransformNode BMGCHFGEDDA;"),
                Target::CreatePattern("RightFoot", "protected ITransformNode AGHJLIMNPJA;"),
                Target::CreatePattern("LeftFoot", "protected ITransformNode FDMBKCKMODA;"),
                Target::CreatePattern("RightWrist", "protected ITransformNode CKABHDJDMAP;"),
                Target::CreatePattern("LeftHand", "protected ITransformNode KOCDBPLKMBI;"),
                Target::CreatePattern("LeftShoulder", "protected ITransformNode LIBEIIIAGIK;"),
                Target::CreatePattern("RightShoulder", "protected ITransformNode HDEPJIBNIIK;"),
                Target::CreatePattern("RightWristJoint", "protected ITransformNode NJDDAPKPILB;"),
                Target::CreatePattern("LeftWristJoint", "protected ITransformNode JHIBMHEMJOL;"),
                Target::CreatePattern("LeftElbow", "protected ITransformNode JBACCHNMGNJ;"),
                Target::CreatePattern("RightElbow", "protected ITransformNode FGECMMJKFNC;"),
            }
        },
    };
}

GameConfig GetFreeFireMaxOffsets() {
    return {
        {
            OffsetCategory::Core,
            {
                Target::CreateFixed("StaticClass", "0x5C"),
                Target::CreateFixed("MatchStatus", "0x3C"),
                Target::CreateFixed("LocalPlayer", "0x7C"),
                Target::CreateFixed("DictionaryEntities", "0x68"),
            }
        },
    };
}

GameConfig GetFreeFireTelaOffsets() {
    return {
        {
            OffsetCategory::Core,
            {
                Target::CreateFixed("StaticClass", "0x5C"),
                Target::CreateFixed("MatchStatus", "0x3C"),
                Target::CreateFixed("LocalPlayer", "0x7C"),
            }
        },
    };
}

std::optional<GameVariant> DetectGameVariant(const std::string& content) {
    if (content.find("FreeFireMAX") != std::string::npos || 
        content.find("MaxGraphics") != std::string::npos) {
        return GameVariant::FreeFireMax;
    }

    if (content.find("FreeFireTELA") != std::string::npos || 
        content.find("TelaVersion") != std::string::npos) {
        return GameVariant::FreeFireTela;
    }

    if (content.find("FreeFire") != std::string::npos || 
        content.find("PlayerNetwork") != std::string::npos) {
        return GameVariant::FreeFire;
    }

    return std::nullopt;
}

}
