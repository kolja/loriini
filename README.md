# Loriini

![latest](https://img.shields.io/github/v/tag/kolja/loriini)
![build](https://github.com/kolja/loriini/actions/workflows/rust.yml/badge.svg)
[![dependency status](https://deps.rs/repo/github/kolja/loriini/status.svg?path=%2F)](https://deps.rs/repo/github/kolja/loriini?path=%2F)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A colorful [parrot](https://en.wikipedia.org/wiki/Loriini); Now also a commandline colorpicker written in Rust.

<img width="431" alt="Screenshot" src="https://user-images.githubusercontent.com/28293/226347553-7e75f345-326a-48b0-910d-0e43411eee74.png">

## commandline options

| Options   |                               | default |
|-----------|-------------------------------|---------|
| -s        | **size**<br/>The height (in lines) which the color picker will occupy  | 12      |
| -x        | **x-factor**<br/>by which the colorwheel is distorted:<br/>  If your font has perfectly square characters, this should be 1   | 0.5     |
| -r        | **outer radius**<br>The radius of the color wheel<br/> It should be half the size to cover all the available area.   | 6      |
| -i, --inner-radius | **inner radius**<br/>The difference between outer radius and inner radius define the width of the color wheel. Zero will give you a solid circle |      |
| -c        | **color**<br/>Initial color, when started (as a hex string) | FF0000      |
| -h, --help        | print **help**|           |
| -v, --version        | print **version**|           |


## usage

Once you have started it, you can use vim-style keybindings to change your color:

| Key |   |
| -- | -- |
| i | **to toggle info**<br/>At startup loriini will only show a colorwheel. To show/hide sliders for lightness and saturation a color-preview with hex-string press "i" |
| j / k | **cycle edit mode**<br/>Select a different edit mode (hue, lightness, saturation with "j" and "k" |
| h / l | **change color**<br/>Change hue, lightness and saturation of your color with "h" and "l" keys |
| y | **"yank"**<br/> "yank" (copy) the selected color to the clipboard |

## demo

https://user-images.githubusercontent.com/28293/226331681-3c681525-68e9-4d5d-8c40-343370e9e555.mov

