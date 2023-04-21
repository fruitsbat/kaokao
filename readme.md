# kaokao
a simple emoji picker using rofi
![a simple usage example, a kaomoji is being copy pasted into a terminal](https://github.com/zoe-bat/kaokao/blob/main/kaokao.gif?raw=true)

## warning
kaomoji are HORRIBLE for screeen readers.

please restrain yourself from using them in more public places like blogs or social media posts

also make sure your friends have no problem reading them when using it with them!

paste responsibly!

## installation
### nix with flakes
if you are using nix flakes, simply add this repository to yours like this:
`flake.nix`:
```
{
  inputs = {
    ...
    kaokao.url = github:zoe-bat/kaokao;
    ...
  };
  outputs =
    { ...
    , kaokao
    , ...
    }@inputs: {
        specialArgs = inputs;
    };
}
```
and then add `pkgs.kaokao.packages.${pkgs.system}.default` to your list of unstalled packages
```
{pkgs, kaokao, ...} : {
  environment.systemPackages = with pkgs; [
    ...
    kaokao.packages.${pkgs.system}.default
    ...
  ]
}
```

### other
you can also manually compile this project

- make sure you have a working install of git, cargo and gcc (skip this step if you are sure you have them)
  - `git --version`
  - `gcc --version`
  - `cargo --version`
- clone this project
  - `git clone https://github.com/zoe-bat/kaokao`
- go into into the folder you just clones
  - `cd kaokao`
- compile it
  - `cargo build --release`

your binary will appear in `kaokao/target/release`

## usage
simply run the binary and a rofi window will pop up!

you do of course need rofi (or a dropin) installed for this to work.

setting the binary to use with `--rofi-binary` is also possible.

### copy to clipboard
kaokao does not provide its own copy to clipboard feature,
but you can simply pipe the output to your clipboard manager of choice

on x11 this is `kaokao | xclip -selection clipboard`
or if you believe that wayland is the future you can do `kaokao | wl-copy`

### type out
depending on your keyboard layout these might require some hacky workarounds 😭

for example see [this](https://github.com/ReimuNotMoe/ydotool#custom-keyboard-layouts)

#### x11
on x11 u can do `moji=$(kaokao) && xdotool type --delay 100 $moji` in bash.

the delay is important since xdotool might not type everything in the right order otherwise

#### everywhere
if you have ydotool set up you can also do
`export YDOTOOL_SOCKET=/tmp/ydotools; moji=$(kaokao) && ydotool type $moji`
make sure to replace `/tmp/ydotools` with wherever your socket actually is

### custom emojis
custom mojis are supported using either json or csv

if no `.json` extension can be found then kaokao will always try to parse them as csv

use your own files `--files something.json something_else.csv`

`something.json` would look like this
```
[
  {
    "value": "hehe",
    "description": "hoho"
  },
  {
    "value": "beep",
    "description": "boop",
    "variations": [
      {
        "value": "beep",
        "description": "boop"
      },
      {
        "value": "bleep",
        "description": "bloop"
      }
    ]
  }
]
```

and `something_else.csv` would be like this
```
value,description
ಠ_ಠ,skeptical
(╯°□°）╯︵ ┻━┻,tableflip
```