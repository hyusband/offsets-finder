#pragma once

#include "models.hpp"
#include <string>
#include <vector>

namespace OffsetsFinder {

bool ExportResults(
    const std::vector<OffsetResult>& results,
    GameVariant game,
    ExportFormat format,
    const std::string& outputPath
);

std::string ExportJson(const std::vector<OffsetResult>& results, GameVariant game);

std::string ExportCppHeader(const std::vector<OffsetResult>& results, GameVariant game);

std::string ExportRustModule(const std::vector<OffsetResult>& results, GameVariant game);

std::string ExportPlainText(const std::vector<OffsetResult>& results, GameVariant game);

}
