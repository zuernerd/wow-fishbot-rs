# WoW Fishing Bot

A fishing bot for World of Warcraft built with Rust and OpenCV.

## ⚠️ Disclaimer

**This project is for educational purposes only.** Using automation tools in World of Warcraft violates the terms of service and can result in account suspension or permanent bans. Use this code at your own risk. You will probably be banned!!

## Overview

**This is a proof of concept (POC) implementation.** The code will likely require adjustments and fine-tuning to work properly on your specific setup. Expect to modify detection thresholds based on your system configuration and game settings.

This fishing bot uses image processing to detect fishing bobbers and automatically responds to fish bites by monitoring visual changes in the game window. It simulates human-like behavior with randomized delays and timing to appear more natural.

## How It Works

1. **Template Loading**: Loads reference images of fishing bobbers from the `./template` directory
2. **Window Capture**: Takes screenshots of the World of Warcraft window
3. **Bobber Detection**: Uses edge detection and template matching to find the bobber
4. **Splash Monitoring**: Compares consecutive frames to detect movement indicating a fish bite
5. **Automated Response**: Right-clicks when a splash is detected to catch the fish
6. **Loop**: Repeats the process with randomized delays

## Prerequisites

- Rust
- OpenCV development libraries

### System Dependencies

This has only been tested on Linux yet. Should run on Windows as well.

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/wow-fishing-bot.git
cd wow-fishing-bot
```

2. Add bobber template images to the `template` directory (see Template Setup below)

3. Build the project:
```bash
cargo build --release
```

## Template Setup

The bot requires template images of fishing bobbers to work properly:

1. Take screenshots of fishing bobbers in various conditions (day/night, different water types)
2. Crop the images to show just the bobber
3. Save them as PNG or JPG files in the `./template` directory
4. The bot will automatically load all images from this directory

### Template Tips
- Use clear, high-contrast images of the bobber
- Capture bobbers in different lighting conditions
- Avoid including too much surrounding water
- Test with multiple templates for better detection

## Configuration

### Key Bindings
- The bot is configured to use **F4** as the fishing key
- Modify the `cast_fishing()` function to change the keybind

### Detection Thresholds
- **Splash threshold**: Currently set to 250 pixels of change
- **Canny edge detection**

Adjust these values in the source code based on your setup and testing.

### Window Setup
- Run World of Warcraft in **windowed mode**
- Ensure the game window is visible and not minimized
- The bot will automatically find the window titled "World of Warcraft"
- **Hide the UI (Alt+z) and zoom into first person view**
- HD Texture packs on vanilla clients can be helpful

## Usage

1. Position your character in front of a fishing spot
2. Ensure you have a fishing pole equipped and fishing skill ready
3. Run the bot:
```bash
cargo run --release
```

4. The bot will:
   - Activate the WoW window (TODO)
   - Start the fishing loop automatically
   - Print status messages to the console

### Stopping the Bot
- Press `Ctrl+C` in the terminal to stop the bot
- Or close the terminal window

## Troubleshooting

### Common Issues

**Bobber detection not working**
- Check that template images are in the `./template` directory
- Ensure templates are clear images of the bobber
- Try capturing templates in similar lighting conditions to your fishing area

**Splash detection too sensitive/not sensitive enough**
- Adjust the threshold value (currently 250) in the `detect_splash()` function
- Test in different water areas and lighting conditions

**Code requires adjustments for your setup**
- Detection thresholds may need tuning based on your display resolution and game settings
- Timing delays might need adjustment depending on your system performance
- Window detection may require different approaches on some systems
- Consider testing in different WoW zones and lighting conditions

## Dependencies

- `xcap` - Screen capture
- `opencv` - Computer vision and image processing
- `enigo` - Cross-platform input simulation
- `xdotool` - Linux-specific window management
- `rand` - Random number generation for human-like delays

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*Remember: Use at your own risk and respect the terms of service of any games you play. Please behave.*