#include "ui.hpp"
#include <iostream>
#include <algorithm>

namespace OffsetsFinder {

void PrintHeader() {
    std::cout << Color::BrightCyan << "╔═══════════════════════════════════════════════════╗" << Color::Reset << "\n";
    std::cout << Color::BrightCyan << Color::Bold << "║     Featured OFFSETS FINDER - C++ EDITION       ║" << Color::Reset << "\n";
    std::cout << Color::BrightCyan << "║              v2.0 - Modular Edition               ║" << Color::Reset << "\n";
    std::cout << Color::BrightCyan << "╚═══════════════════════════════════════════════════╝" << Color::Reset << "\n\n";
}

void PrintGameMenu() {
    std::cout << Color::BrightBlue << Color::Bold << "Select Free Fire variant:" << Color::Reset << "\n";
    std::cout << "  " << Color::BrightYellow << "1." << Color::Reset << " Free Fire (Standard)\n";
    std::cout << "  " << Color::BrightYellow << "2." << Color::Reset << " Free Fire MAX\n";
    std::cout << "  " << Color::BrightYellow << "3." << Color::Reset << " Free Fire TELA\n";
    std::cout << "  " << Color::BrightYellow << "4." << Color::Reset << " Auto-detect from dump\n\n";
}

int GetGameSelection() {
    std::cout << Color::BrightBlue << "Enter your choice (1-4): " << Color::Reset;
    int choice;
    std::cin >> choice;
    std::cin.ignore();
    return choice;
}

void PrintResults(const std::vector<OffsetResult>& results, GameVariant game) {
    std::cout << "\n" << Color::BrightCyan << Color::Bold 
              << "====== " << GameVariantToString(game) << " OFFSETS ======" 
              << Color::Reset << "\n";
    
    std::string currentCategory;
    for (const auto& result : results) {
        if (result.category != currentCategory) {
            std::cout << "\n" << Color::BrightMagenta << Color::Bold 
                      << "--- " << result.category << " ---" 
                      << Color::Reset << "\n";
            currentCategory = result.category;
        }
        
        if (result.offset.has_value()) {
            std::cout << Color::BrightGreen << Color::Bold << result.name << Color::Reset 
                      << " " << Color::BrightYellow << result.offset.value() << Color::Reset << "\n";
        } else {
            std::cout << Color::BrightRed << Color::Bold << result.name << Color::Reset 
                      << " " << Color::BrightRed << "NOT FOUND" << Color::Reset << "\n";
        }
    }
}

void PrintStatistics(const std::vector<OffsetResult>& results) {
    size_t found = std::count_if(results.begin(), results.end(),
        [](const auto& r) { return r.offset.has_value(); });
    size_t total = results.size();
    size_t missing = total - found;
    
    std::cout << "\n" << Color::BrightCyan << "═══════════════════════════════════════════" << Color::Reset << "\n";
    std::cout << Color::BrightCyan << Color::Bold << "STATISTICS" << Color::Reset << "\n";
    std::cout << Color::BrightCyan << "═══════════════════════════════════════════" << Color::Reset << "\n";
    std::cout << Color::BrightWhite << "Total offsets: " << Color::Reset 
              << Color::BrightYellow << Color::Bold << total << Color::Reset << "\n";
    std::cout << Color::BrightGreen << Color::Bold << "Found: " << Color::Reset 
              << Color::BrightGreen << Color::Bold << found << Color::Reset << "\n";
    std::cout << Color::BrightRed << Color::Bold << "Missing: " << Color::Reset 
              << Color::BrightRed << Color::Bold << missing << Color::Reset << "\n";
    
    float percentage = (static_cast<float>(found) / total) * 100.0f;
    std::cout << Color::BrightWhite << "Success rate: " << Color::Reset 
              << Color::BrightYellow << Color::Bold << std::fixed << std::setprecision(1) 
              << percentage << "%" << Color::Reset << "\n";
    std::cout << Color::BrightCyan << "═══════════════════════════════════════════" << Color::Reset << "\n";
}

void PrintExportMenu() {
    std::cout << "\n" << Color::BrightBlue << Color::Bold << "Export options:" << Color::Reset << "\n";
    std::cout << "  " << Color::BrightYellow << "1." << Color::Reset << " JSON format\n";
    std::cout << "  " << Color::BrightYellow << "2." << Color::Reset << " C++ header (.hpp)\n";
    std::cout << "  " << Color::BrightYellow << "3." << Color::Reset << " Rust module (.rs)\n";
    std::cout << "  " << Color::BrightYellow << "4." << Color::Reset << " Plain text (.txt)\n";
    std::cout << "  " << Color::BrightYellow << "5." << Color::Reset << " Skip export\n\n";
}

int GetExportSelection() {
    std::cout << Color::BrightBlue << "Enter your choice (1-5): " << Color::Reset;
    int choice;
    std::cin >> choice;
    std::cin.ignore();
    return choice;
}

void PrintSuccess(const std::string& message) {
    std::cout << Color::BrightGreen << Color::Bold << "✓ " << Color::Reset 
              << Color::BrightWhite << message << Color::Reset << "\n";
}

void PrintError(const std::string& message) {
    std::cerr << Color::BrightRed << Color::Bold << "✗ " << Color::Reset 
              << Color::BrightRed << message << Color::Reset << "\n";
}

void PrintInfo(const std::string& message) {
    std::cout << Color::BrightBlue << Color::Bold << "ℹ " << Color::Reset 
              << Color::BrightWhite << message << Color::Reset << "\n";
}

void WaitForEnter() {
    std::cout << "\n" << Color::BrightBlue << "Press Enter to exit..." << Color::Reset;
    std::cin.get();
}

}
