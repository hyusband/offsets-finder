#include "scanner.hpp"
#include <fstream>
#include <sstream>
#include <regex>
#include <algorithm>

namespace OffsetsFinder {

std::optional<std::string> ExtractHex(const std::string& line) {
    std::regex hexPattern("0x[0-9A-Fa-f]+");
    std::smatch match;
    
    if (std::regex_search(line, match, hexPattern)) {
        return match.str();
    }
    
    return std::nullopt;
}

std::vector<OffsetResult> ScanFile(const std::string& filePath, const GameConfig& config) {
    std::vector<OffsetResult> results;
    
    std::ifstream file(filePath);
    if (!file.is_open()) {
        return results;
    }
    
    std::vector<std::string> lines;
    std::string line;
    while (std::getline(file, line)) {
        lines.push_back(line);
    }
    file.close();
    
    for (const auto& [category, targets] : config) {
        std::string categoryName = OffsetCategoryToString(category);
        
        for (const auto& target : targets) {
            if (target.GetType() == Target::Type::Separator) {
                continue;
            }
            
            std::optional<std::string> offset;
            
            if (target.GetType() == Target::Type::Fixed) {
                offset = target.GetHex();
            } else if (target.GetType() == Target::Type::Pattern) {
                offset = FindPatternInLines(lines, target.GetPattern());
            }
            
            results.emplace_back(target.GetName(), offset, categoryName);
        }
    }
    
    return results;
}

std::optional<std::string> FindPatternInLines(const std::vector<std::string>& lines, const std::string& pattern) {
    for (const auto& line : lines) {
        if (line.find(pattern) != std::string::npos) {
            auto hex = ExtractHex(line);
            if (hex.has_value()) {
                return hex;
            }
        }
    }
    return std::nullopt;
}

}
