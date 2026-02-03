#include "models.hpp"

namespace OffsetsFinder {

Target Target::CreateFixed(const std::string& name, const std::string& hex) {
    return Target(Type::Fixed, name, hex);
}

Target Target::CreatePattern(const std::string& name, const std::string& pattern) {
    return Target(Type::Pattern, name, pattern);
}

Target Target::CreateSeparator() {
    return Target(Type::Separator, "", "");
}

Target::Target(Type type, const std::string& name, const std::string& value)
    : type_(type), name_(name) {
    if (type == Type::Fixed) {
        hex_ = value;
    } else if (type == Type::Pattern) {
        pattern_ = value;
    }
}

std::string GameVariantToString(GameVariant variant) {
    switch (variant) {
        case GameVariant::FreeFire: return "Free Fire";
        case GameVariant::FreeFireMax: return "Free Fire MAX";
        case GameVariant::FreeFireTela: return "Free Fire TELA";
        default: return "Unknown";
    }
}

std::string OffsetCategoryToString(OffsetCategory category) {
    switch (category) {
        case OffsetCategory::Core: return "Core";
        case OffsetCategory::Player: return "Player";
        case OffsetCategory::Camera: return "Camera";
        case OffsetCategory::Weapon: return "Weapon";
        case OffsetCategory::Silent: return "Silent Aim";
        case OffsetCategory::Collision: return "Collision";
        case OffsetCategory::Attributes: return "Attributes";
        case OffsetCategory::Bot: return "Bot Detection";
        case OffsetCategory::Skeleton: return "Skeleton/Bones";
        default: return "Unknown";
    }
}

std::string ExportFormatExtension(ExportFormat format) {
    switch (format) {
        case ExportFormat::Json: return "json";
        case ExportFormat::CppHeader: return "hpp";
        case ExportFormat::RustModule: return "rs";
        case ExportFormat::PlainText: return "txt";
        default: return "txt";
    }
}

}
