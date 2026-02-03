#pragma once

#include "models.hpp"
#include "config.hpp"
#include <string>
#include <vector>

namespace OffsetsFinder {

std::optional<std::string> ExtractHex(const std::string& line);

std::vector<OffsetResult> ScanFile(const std::string& filePath, const GameConfig& config);

std::optional<std::string> FindPatternInLines(const std::vector<std::string>& lines, const std::string& pattern);

}
