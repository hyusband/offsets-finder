#include "models.hpp"
#include "config.hpp"
#include "scanner.hpp"
#include "exporter.hpp"
#include "ui.hpp"
#include <iostream>
#include <fstream>
#include <sstream>

#ifdef _WIN32
#include <windows.h>
#endif

using namespace OffsetsFinder;

void EnableAnsiColors() {
#ifdef _WIN32
    HANDLE hOut = GetStdHandle(STD_OUTPUT_HANDLE);
    DWORD dwMode = 0;
    GetConsoleMode(hOut, &dwMode);
    dwMode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    SetConsoleMode(hOut, dwMode);
#endif
}

std::string GetFilePath() {
    std::string path;
    std::cout << Color::BrightBlue << "Enter path to dump.cs file: " << Color::Reset;
    std::getline(std::cin, path);
    return path;
}

std::string GetOutputPath(const std::string& extension) {
    std::string path;
    std::cout << Color::BrightBlue << "Enter output file path (or press Enter for default): " << Color::Reset;
    std::getline(std::cin, path);
    
    if (path.empty()) {
        path = "offsets." + extension;
    }
    
    return path;
}

int main() {
    EnableAnsiColors();
    PrintHeader();
    
    PrintInfo("Enter the path to your dump.cs file");
    std::string filePath = GetFilePath();
    
    std::ifstream testFile(filePath);
    if (!testFile.is_open()) {
        PrintError("File not found: " + filePath);
        WaitForEnter();
        return 1;
    }
    testFile.close();
    
    PrintSuccess("File loaded: " + filePath);
    
    PrintGameMenu();
    int gameChoice = GetGameSelection();
    
    GameVariant gameVariant;
    if (gameChoice == 4) {
        PrintInfo("Auto-detecting game variant...");
        std::ifstream file(filePath);
        std::stringstream buffer;
        buffer << file.rdbuf();
        std::string content = buffer.str();
        file.close();
        
        auto detected = DetectGameVariant(content);
        if (detected.has_value()) {
            gameVariant = detected.value();
            PrintSuccess("Detected: " + GameVariantToString(gameVariant));
        } else {
            PrintError("Could not detect game variant. Using Free Fire standard.");
            gameVariant = GameVariant::FreeFire;
        }
    } else {
        switch (gameChoice) {
            case 1: gameVariant = GameVariant::FreeFire; break;
            case 2: gameVariant = GameVariant::FreeFireMax; break;
            case 3: gameVariant = GameVariant::FreeFireTela; break;
            default:
                PrintError("Invalid selection. Using Free Fire standard.");
                gameVariant = GameVariant::FreeFire;
        }
    }
    
    PrintInfo("Scanning for " + GameVariantToString(gameVariant) + " offsets...");
    
    auto config = GetGameConfig(gameVariant);
    auto results = ScanFile(filePath, config);
    
    if (results.empty()) {
        PrintError("Scan failed or no results found.");
        WaitForEnter();
        return 1;
    }
    
    PrintResults(results, gameVariant);
    PrintStatistics(results);
    
    PrintExportMenu();
    int exportChoice = GetExportSelection();
    
    if (exportChoice >= 1 && exportChoice <= 4) {
        ExportFormat format;
        std::string extension;
        
        switch (exportChoice) {
            case 1:
                format = ExportFormat::Json;
                extension = "json";
                break;
            case 2:
                format = ExportFormat::CppHeader;
                extension = "hpp";
                break;
            case 3:
                format = ExportFormat::RustModule;
                extension = "rs";
                break;
            case 4:
                format = ExportFormat::PlainText;
                extension = "txt";
                break;
        }
        
        std::string outputPath = GetOutputPath(extension);
        
        if (ExportResults(results, gameVariant, format, outputPath)) {
            PrintSuccess("Exported to: " + outputPath);
        } else {
            PrintError("Export failed: " + outputPath);
        }
    } else {
        PrintInfo("Skipping export.");
    }
    
    WaitForEnter();
    return 0;
}
