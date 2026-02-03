#include "exporter.hpp"
#include <fstream>
#include <sstream>
#include <map>
#include <chrono>
#include <iomanip>
#include <algorithm>

namespace OffsetsFinder {

bool ExportResults(
    const std::vector<OffsetResult>& results,
    GameVariant game,
    ExportFormat format,
    const std::string& outputPath
) {
    std::string content;
    
    switch (format) {
        case ExportFormat::Json:
            content = ExportJson(results, game);
            break;
        case ExportFormat::CppHeader:
            content = ExportCppHeader(results, game);
            break;
        case ExportFormat::RustModule:
            content = ExportRustModule(results, game);
            break;
        case ExportFormat::PlainText:
            content = ExportPlainText(results, game);
            break;
    }
    
    std::ofstream file(outputPath);
    if (!file.is_open()) {
        return false;
    }
    
    file << content;
    file.close();
    return true;
}

std::string ExportJson(const std::vector<OffsetResult>& results, GameVariant game) {
    std::ostringstream oss;
    
    std::map<std::string, std::map<std::string, std::string>> offsetsByCategory;
    for (const auto& result : results) {
        if (result.offset.has_value()) {
            offsetsByCategory[result.category][result.name] = result.offset.value();
        }
    }
    
    size_t found = std::count_if(results.begin(), results.end(),
        [](const auto& r) { return r.offset.has_value(); });
    
    oss << "{\n";
    oss << "  \"game\": \"" << GameVariantToString(game) << "\",\n";
    oss << "  \"offsets\": {\n";
    
    bool firstCategory = true;
    for (const auto& [category, offsets] : offsetsByCategory) {
        if (!firstCategory) oss << ",\n";
        firstCategory = false;
        
        oss << "    \"" << category << "\": {\n";
        bool firstOffset = true;
        for (const auto& [name, offset] : offsets) {
            if (!firstOffset) oss << ",\n";
            firstOffset = false;
            oss << "      \"" << name << "\": \"" << offset << "\"";
        }
        oss << "\n    }";
    }
    
    oss << "\n  },\n";
    oss << "  \"statistics\": {\n";
    oss << "    \"total\": " << results.size() << ",\n";
    oss << "    \"found\": " << found << ",\n";
    oss << "    \"missing\": " << (results.size() - found) << "\n";
    oss << "  }\n";
    oss << "}\n";
    
    return oss.str();
}

std::string ExportCppHeader(const std::vector<OffsetResult>& results, GameVariant game) {
    std::ostringstream oss;
    
    std::string namespaceName;
    switch (game) {
        case GameVariant::FreeFire: namespaceName = "FreeFire"; break;
        case GameVariant::FreeFireMax: namespaceName = "FreeFireMax"; break;
        case GameVariant::FreeFireTela: namespaceName = "FreeFireTela"; break;
    }
    
    oss << "// Auto-generated offsets\n";
    oss << "// Game: " << GameVariantToString(game) << "\n\n";
    oss << "#pragma once\n";
    oss << "#include <cstdint>\n\n";
    oss << "namespace " << namespaceName << " {\n";
    
    std::string currentCategory;
    for (const auto& result : results) {
        if (result.category != currentCategory) {
            if (!currentCategory.empty()) oss << "\n";
            oss << "    // " << result.category << "\n";
            currentCategory = result.category;
        }
        
        if (result.offset.has_value()) {
            std::string constName = result.name;
            std::transform(constName.begin(), constName.end(), constName.begin(), ::toupper);
            std::replace(constName.begin(), constName.end(), ' ', '_');
            
            oss << "    constexpr uintptr_t " << constName << " = " << result.offset.value() << ";\n";
        }
    }
    
    oss << "}\n";
    return oss.str();
}

std::string ExportRustModule(const std::vector<OffsetResult>& results, GameVariant game) {
    std::ostringstream oss;
    
    std::string moduleName;
    switch (game) {
        case GameVariant::FreeFire: moduleName = "freefire"; break;
        case GameVariant::FreeFireMax: moduleName = "freefire_max"; break;
        case GameVariant::FreeFireTela: moduleName = "freefire_tela"; break;
    }
    
    oss << "// Auto-generated offsets\n";
    oss << "// Game: " << GameVariantToString(game) << "\n\n";
    oss << "#![allow(dead_code)]\n\n";
    oss << "pub mod " << moduleName << " {\n";
    
    std::string currentCategory;
    for (const auto& result : results) {
        if (result.category != currentCategory) {
            if (!currentCategory.empty()) oss << "\n";
            oss << "    // " << result.category << "\n";
            currentCategory = result.category;
        }
        
        if (result.offset.has_value()) {
            std::string constName = result.name;
            std::transform(constName.begin(), constName.end(), constName.begin(), ::toupper);
            std::replace(constName.begin(), constName.end(), ' ', '_');
            std::replace(constName.begin(), constName.end(), '-', '_');
            
            oss << "    pub const " << constName << ": usize = " << result.offset.value() << ";\n";
        }
    }
    
    oss << "}\n";
    return oss.str();
}

std::string ExportPlainText(const std::vector<OffsetResult>& results, GameVariant game) {
    std::ostringstream oss;
    
    oss << "====== " << GameVariantToString(game) << " OFFSETS ======\n\n";
    
    std::string currentCategory;
    for (const auto& result : results) {
        if (result.category != currentCategory) {
            if (!currentCategory.empty()) oss << "\n";
            oss << "--- " << result.category << " ---\n";
            currentCategory = result.category;
        }
        
        oss << result.name << " = ";
        if (result.offset.has_value()) {
            oss << result.offset.value();
        } else {
            oss << "NOT FOUND";
        }
        oss << "\n";
    }
    
    size_t found = std::count_if(results.begin(), results.end(),
        [](const auto& r) { return r.offset.has_value(); });
    
    oss << "\n====== STATISTICS ======\n";
    oss << "Found: " << found << "/" << results.size() << "\n";
    
    return oss.str();
}

}
