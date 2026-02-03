# Free Fire Offsets Finder - C++ Edition

A high-performance C++ implementation of the Free Fire offsets finder tool. Supports Free Fire, Free Fire MAX, and Free Fire TELA with multiple export formats.

## ✨ Features

- 🎮 **Multi-Game Support** - Free Fire, Free Fire MAX, Free Fire TELA
- 🔍 **Auto-Detection** - Automatically detect game variant from dump files
- 🎨 **Colored Output** - ANSI colored terminal interface
- 📊 **Multiple Export Formats** - JSON, C++ headers, Rust modules, Plain text
- ⚡ **High Performance** - Optimized C++17 implementation
- 🏗️ **Modular Architecture** - Clean, maintainable codebase

## 🚀 Build Instructions

### Windows (Visual Studio)

```bash
mkdir build
cd build
cmake ..
cmake --build . --config Release
```

### Linux/macOS

```bash
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
make
```

## 📖 Usage

Run the executable:

```bash
./bin/offsets-finder
```

**Interactive Steps:**

1. Enter path to dump.cs file
2. Select game variant (or auto-detect)
3. View colored results by category
4. Choose export format (optional)

## 🏗️ Project Structure

```
cpp/
├── include/
│   ├── models.hpp      # Data structures and enums
│   ├── config.hpp      # Game configurations
│   ├── scanner.hpp     # File scanning logic
│   ├── exporter.hpp    # Export functionality
│   └── ui.hpp          # Terminal UI
├── src/
│   ├── main.cpp        # Entry point
│   ├── models.cpp
│   ├── config.cpp
│   ├── scanner.cpp
│   ├── exporter.cpp
│   └── ui.cpp
├── CMakeLists.txt
└── README.md
```

## 🎯 Offset Categories

The tool extracts **48 offsets** across 9 categories:

1. **Core** (5) - StaticClass, MatchStatus, LocalPlayer, etc.
2. **Player** (9) - IsDead, Name, Avatar, Position, etc.
3. **Camera** (4) - FollowCamera, AimRotation, Transform
4. **Weapon** (4) - Weapon, WeaponData, Recoil, ViewMatrix
5. **Silent Aim** (4) - Silent1-4
6. **Collision** (1) - HeadCollider
7. **Attributes** (2) - PlayerAttributes, NoReload
8. **Bot Detection** (1) - isBot
9. **Skeleton** (17) - Head, Root, Limbs, Joints

## 📊 Export Formats

### JSON

```json
{
  "game": "Free Fire",
  "offsets": {
    "Player": {
      "Player_IsDead": "0x4C"
    }
  }
}
```

### C++ Header

```cpp
namespace FreeFire {
    constexpr uintptr_t PLAYER_ISDEAD = 0x4C;
}
```

### Rust Module

```rust
pub mod freefire {
    pub const PLAYER_ISDEAD: usize = 0x4C;
}
```

## 🔧 Requirements

- CMake 3.15+
- C++17 compatible compiler
  - MSVC 2017+ (Windows)
  - GCC 7+ (Linux)
  - Clang 5+ (macOS)

## 📝 Adding New Offsets

Edit `src/config.cpp`:

```cpp
GameConfig GetFreeFireOffsets() {
    return {
        {
            OffsetCategory::Player,
            {
                Target::CreateFixed("NewOffset", "0xABC"),
                Target::CreatePattern("DynamicOffset", "pattern to search"),
            }
        },
    };
}
```

## 🤝 Contributing

Contributions welcome! Please ensure:

- Code follows C++17 standards
- Proper error handling
- Documentation for new features

## 📝 License

Apache License 2.0

---

**Made with ❤️ in C++ ⚙️**
