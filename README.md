# Free Fire Offsets Finder - Rust Edition v2.0

A powerful, modular tool for extracting memory offsets from Free Fire dump files. Supports **Free Fire**, **Free Fire MAX**, and **Free Fire TELA** with multiple export formats.

## ✨ Features

### Core Capabilities

- 🎮 **Multi-Game Support** - Free Fire, Free Fire MAX, Free Fire TELA
- 🔍 **Auto-Detection** - Automatically detect game variant from dump files
- 🎨 **Beautiful UI** - Colored terminal output with categories and statistics
- 📊 **Multiple Export Formats** - JSON, C++ headers, Rust modules, Plain text
- ⚡ **Blazing Fast** - Rust performance with optimized builds
- 🏗️ **Modular Architecture** - Clean, maintainable codebase

### Export Formats

#### JSON

```json
{
  "game": "Free Fire",
  "offsets": {
    "Player": {
      "Player_IsDead": "0x4C",
      "Player_Name": "0x128"
    }
  }
}
```

#### C++ Header

```cpp
namespace FreeFire {
    constexpr uintptr_t PLAYER_ISDEAD = 0x4C;
    constexpr uintptr_t PLAYER_NAME = 0x128;
}
```

#### Rust Module

```rust
pub mod freefire {
    pub const PLAYER_ISDEAD: usize = 0x4C;
    pub const PLAYER_NAME: usize = 0x128;
}
```

## 🚀 Installation

### From Source

1. Install [Rust](https://rustup.rs/)
2. Clone and build:
   ```bash
   git clone <your-repo>
   cd offsets-finder/offsets-finder
   cargo build --release
   ```
3. Executable: `target/release/offsets-finder.exe`

## 📖 Usage

### Interactive Mode (GUI)

Simply run the executable:

```bash
./offsets-finder.exe
```

**Steps:**

1. Select dump.cs file via GUI dialog
2. Choose game variant (or auto-detect)
3. View results with colored output
4. Export to your preferred format

### CLI Mode

```bash
# Scan with auto-detection
offsets-finder scan --file dump.cs

# Specify game variant
offsets-finder scan --file dump.cs --game max

# Export to JSON
offsets-finder scan --file dump.cs --export json --output offsets.json

# Export to C++ header
offsets-finder scan --file dump.cs --game freefire --export cpp --output offsets.hpp
```

#### CLI Arguments

| Argument   | Options                           | Description          |
| ---------- | --------------------------------- | -------------------- |
| `--file`   | path                              | Path to dump.cs file |
| `--game`   | `freefire`, `max`, `tela`, `auto` | Game variant         |
| `--export` | `json`, `cpp`, `rust`, `txt`      | Export format        |
| `--output` | path                              | Output file path     |

## 🎯 Offset Categories

The tool extracts offsets across 9 categories:

1. **Core** - StaticClass, MatchStatus, LocalPlayer
2. **Player** - Health, Name, Avatar, Position
3. **Camera** - FollowCamera, AimRotation, Transform
4. **Weapon** - WeaponData, Recoil, ViewMatrix
5. **Silent Aim** - Silent aim offsets
6. **Collision** - HeadCollider
7. **Attributes** - PlayerAttributes, NoReload
8. **Bot Detection** - isBot
9. **Skeleton** - Head, Spine, Limbs (17 bones)

## 🏗️ Architecture

```
src/
├── main.rs       # Entry point, CLI handling
├── models.rs     # Data structures (Target, OffsetResult, etc.)
├── config.rs     # Game-specific offset configurations
├── scanner.rs    # File scanning and pattern matching
├── exporter.rs   # Export to multiple formats
└── ui.rs         # Terminal UI and user interaction
```

### Adding New Offsets

Edit `src/config.rs`:

```rust
fn get_freefire_offsets() -> Vec<(OffsetCategory, Vec<Target>)> {
    vec![
        (
            OffsetCategory::Player,
            vec![
                Target::new_fixed("NewOffset", "0xABC"),
                Target::new_pattern("DynamicOffset", "pattern to search"),
            ],
        ),
    ]
}
```

### Adding New Game Variants

1. Add variant to `GameVariant` enum in `models.rs`
2. Create offset configuration in `config.rs`
3. Update detection logic in `detect_game_variant()`

## 🔧 Configuration

### Custom Patterns

Create a `config.toml` file (future feature):

```toml
[freefire.custom]
MyOffset = { pattern = "custom pattern", category = "Player" }
```

## 📊 Performance

| Metric       | Value   |
| ------------ | ------- |
| Scan Time    | ~10ms   |
| Memory Usage | ~5MB    |
| Binary Size  | ~2MB    |
| Startup Time | Instant |

## 🤝 Contributing

To add support for new Free Fire versions:

1. Obtain dump.cs file
2. Add patterns to `config.rs`
3. Test with real dumps
4. Submit PR

## 📝 License

Apache License 2.0

## 🙏 Credits

- Original Python implementation: [luciviq3439](https://github.com/luciviq3439/Offsets-Finder-By-luciviq3439)
- Rust rewrite: Modular architecture with Free Fire focus

---

**Made with ❤️ and 🦀 Rust**

A powerful and fast tool for extracting memory offsets from C# dump files, written in Rust. This is a complete rewrite of the Python-based offsets finder with improved performance, better error handling, and a modern terminal UI.

## Features

✨ **Modern GUI File Picker** - Easy file selection with native dialogs  
🎨 **Colored Terminal Output** - Beautiful, readable results with syntax highlighting  
⚡ **Blazing Fast** - Rust performance for instant offset extraction  
🔍 **Regex Pattern Matching** - Flexible pattern search with hex extraction  
📊 **Statistics** - Shows found/total offset counts  
🛡️ **Error Handling** - Robust file reading and pattern matching

## Installation

### From Source

1. Make sure you have [Rust installed](https://rustup.rs/)
2. Clone this repository:
   ```bash
   git clone <your-repo-url>
   cd offsets-finder/offsets-finder
   ```
3. Build the release version:
   ```bash
   cargo build --release
   ```
4. The executable will be in `target/release/offsets-finder.exe`

## Usage

1. Run the executable:

   ```bash
   ./target/release/offsets-finder.exe
   ```

   Or simply double-click `offsets-finder.exe`

2. A file picker dialog will appear - select your `dump.cs` file

3. The tool will search for all configured offsets and display results with:
   - ✅ **Green** - Successfully found offsets
   - ❌ **Red** - Offsets not found
   - 📊 Statistics showing how many offsets were found

## Configuration

To modify the target offsets, edit the `get_targets()` function in `src/main.rs`:

```rust
fn get_targets() -> Vec<Target> {
    vec![
        // Fixed hex offsets
        Target::new_fixed("StaticClass", "0x5C"),

        // Pattern-based search
        Target::new_pattern("Player_Name", "protected string OIAJCBLDHKP;"),

        // Visual separator
        Target::Separator,
    ]
}
```

### Target Types

- **Fixed**: `Target::new_fixed(name, hex_value)` - Static hex offsets
- **Pattern**: `Target::new_pattern(name, search_pattern)` - Searches dump file for pattern and extracts hex
- **Separator**: `Target::Separator` - Adds visual spacing in output

## Building for Distribution

For the smallest, most optimized executable:

```bash
cargo build --release
```

The release profile is configured for maximum optimization:

- LTO (Link Time Optimization) enabled
- Minimal code size (`opt-level = "z"`)
- Single codegen unit
- Debug symbols stripped

## Dependencies

- `regex` - Pattern matching and hex extraction
- `rfd` - Native file picker dialogs
- `colored` - Terminal color output

## Comparison with Python Version

| Feature         | Python           | Rust                |
| --------------- | ---------------- | ------------------- |
| Performance     | ~100ms           | ~10ms               |
| Memory Usage    | ~50MB            | ~5MB                |
| Executable Size | Requires Python  | ~2MB standalone     |
| Startup Time    | Slow             | Instant             |
| Dependencies    | Python + tkinter | None (standalone)   |
| Error Handling  | Basic            | Comprehensive       |
| Terminal UI     | Plain text       | Colored & formatted |

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.

## Credits

Inspired by the original Python implementation by [luciviq3439](https://github.com/luciviq3439/Offsets-Finder-By-luciviq3439)

---

**Made with ❤️ and 🦀 Rust**
