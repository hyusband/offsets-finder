cmake_minimum_required(VERSION 3.15)
project(OffsetsFinder VERSION 2.0 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Output directories
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

# Include directories
include_directories(${CMAKE_SOURCE_DIR}/include)

# Source files
set(SOURCES
    src/main.cpp
    src/models.cpp
    src/config.cpp
    src/scanner.cpp
    src/exporter.cpp
    src/ui.cpp
)

# Header files
set(HEADERS
    include/models.hpp
    include/config.hpp
    include/scanner.hpp
    include/exporter.hpp
    include/ui.hpp
)

# Create executable
add_executable(offsets-finder ${SOURCES} ${HEADERS})

# Compiler options
if(MSVC)
    target_compile_options(offsets-finder PRIVATE /W4 /O2)
    # Enable ANSI colors on Windows
    target_compile_definitions(offsets-finder PRIVATE _CRT_SECURE_NO_WARNINGS)
else()
    target_compile_options(offsets-finder PRIVATE -Wall -Wextra -O3)
endif()

# Installation
install(TARGETS offsets-finder DESTINATION bin)

# Print build information
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "C++ Compiler: ${CMAKE_CXX_COMPILER}")
message(STATUS "C++ Standard: ${CMAKE_CXX_STANDARD}")
