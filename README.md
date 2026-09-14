# Pokemon Bank Offline Mod

An offline mod for the original Pokemon Bank software, preserving its functionality beyond the shutdown of Nintendo's online services.

## User Features

- **Migrate Mode** – The first time you use the mod on a physical console, it will guide you through migrating from Online Bank to Offline Bank so you don't have to move all of your Pokemon manually.
- **Offline Mode** – Use Pokemon Bank without an internet connection.
- **Mode Display** – The current mode is always displayed so you're never confused about if the mod is loaded or what it's doing.
- **Debug View** – Press `Start + Up` to view additional information about what Bank is doing.
- **Emulator Support** – The mod skips Migrate Mode and starts working offline instantly.
- **Pass Time Remaining Display** – I figured some people would be uneasy if Bank said "your pass/trial is about to expire" in red text, so I made it always show the expiration date a long ways out. Unimportant, but a nice detail.

## Safety features:

- **Network stubs** – This plugin stubs functions that involve network communication to make sure we're not accidentally sending data to official servers.
- **Bank Save Redirect** – The Bank save (`turtle`), which has things like server provided data, is redirected to the SD. This ensures your actual Bank save is untouched in case you want to switch to online mode again.
- **Stubbed Move To Home Button** – Obviously an offline bank can't transfer to home, so this button will just exit bank. Pressing this didn't cause issues during testing, but I wanted to prevent unexpected scenarios.
- **Stubbed Download Transporter Button** – Similar to the above, this wasn't an issue while testing, but I wanted to prevent odd scenarios.

## Before You Start

**Back up your saves first.** Make sure you have backups of:

- Your Pokemon Bank save
- Any 3DS Pokemon game saves you plan to use

The plugin is provided as-is. I’m not responsible for data loss, online bans, or other issues that may occur while using it.

With that said, multiple people have tested this without any issues.

## Setup

1. Download `plugin.3gx` from the releases
2. Copy it to: `/luma/plugins/00040000000C9B00/plugin.3gx`
3. Launch Pokemon Bank

## Known issues

- The background music doesn't change

## Building

1. Install rust and the armv6k-nintendo-3ds target, devkitarm, and [3gxtool](https://gitlab.com/thepixellizeross/3gxtool)
1. Run `make`

## Credits

- [libctru](https://github.com/devkitPro/libctru) for making REing easier and some of the logic in this project
- [ntr_overlay_samples](https://github.com/44670/ntr_overlay_samples) for print/display logic
- [ctrpluginframework](https://gitlab.com/thepixellizeross/ctrpluginframework) for plgldr logic and the test plugin
