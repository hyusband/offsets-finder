#pragma once

#include <string>
#include <vector>
#include <optional>
#include <memory>

namespace OffsetsFinder {

enum class GameVariant {
    FreeFire,
    FreeFireMax,
    FreeFireTela
};

enum class OffsetCategory {
    Core,
    Player,
    Camera,
    Weapon,
    Silent,
    Collision,
    Attributes,
    Bot,
    Skeleton
};

enum class ExportFormat {
    Json,
    CppHeader,
    RustModule,
    PlainText
};

class Target {
public:
    enum class Type {
        Fixed,
        Pattern,
        Separator
    };

    static Target CreateFixed(const std::string& name, const std::string& hex);
    static Target CreatePattern(const std::string& name, const std::string& pattern);
    static Target CreateSeparator();

    Type GetType() const { return type_; }
    const std::string& GetName() const { return name_; }
    const std::string& GetHex() const { return hex_; }
    const std::string& GetPattern() const { return pattern_; }

private:
    Target(Type type, const std::string& name, const std::string& value);
    
    Type type_;
    std::string name_;
    std::string hex_;
    std::string pattern_;
};

struct OffsetResult {
    std::string name;
    std::optional<std::string> offset;
    std::string category;

    OffsetResult(const std::string& n, const std::optional<std::string>& o, const std::string& c)
        : name(n), offset(o), category(c) {}
};

std::string GameVariantToString(GameVariant variant);
std::string OffsetCategoryToString(OffsetCategory category);
std::string ExportFormatExtension(ExportFormat format);

}
