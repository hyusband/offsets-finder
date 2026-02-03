#pragma once

#include "models.hpp"
#include <string>
#include <vector>

namespace OffsetsFinder {

namespace Color {
    const std::string Reset = "\033[0m";
    const std::string BrightCyan = "\033[96m";
    const std::string BrightGreen = "\033[92m";
    const std::string BrightRed = "\033[91m";
    const std::string BrightYellow = "\033[93m";
    const std::string BrightBlue = "\033[94m";
    const std::string BrightMagenta = "\033[95m";
    const std::string BrightWhite = "\033[97m";
    const std::string Bold = "\033[1m";
}

void PrintHeader();

void PrintGameMenu();

int GetGameSelection();

void PrintResults(const std::vector<OffsetResult>& results, GameVariant game);

void PrintStatistics(const std::vector<OffsetResult>& results);

void PrintExportMenu();

int GetExportSelection();

void PrintSuccess(const std::string& message);

void PrintError(const std::string& message);

void PrintInfo(const std::string& message);

void WaitForEnter();

}
