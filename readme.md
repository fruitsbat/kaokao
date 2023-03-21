# kaokao
a simple emoji picker using rofi
![a simple usage example, a kaomoji is being copy pasted into a terminal](https://github.com/zoe-bat/kaokao/blob/main/kaokao.gif?raw=true)

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
make sure you have a working install of cargo and run `cargo build --release`
a compiled binary will appear in `kaokao/target/release`

## usage
simply run the binary and a rofi window will pop up!
you do of course need rofi (or a dropin) installed for this to work.
setting the binary to use with `--rofi-binary` is also possible.

### copy to clipboard
kaokao does not provide its own copy to clipboard feature,
but you can simply pipe the output to your clipboard manager of choice

on x11 this is `kaokao | xclip -selection clipboard`
or if you believe that wayland is the future you can do `kaokao | wl-copy`

## type out
on x11 u can do `moji=$(kaokao) && xdotool type --delay 100 $moji` in bash.
the delay is important since xdotool might not type everything in the right order otherwise

### custom emojis
custom mojis are supported using either json or csv
if no `.json` extension can be found then kaokao will always try to parse them as csv
use your own files `--files something.json something_else.csv`

`something.json` would look like this
```
[
{"value" : " ʅ(ツ)ʃ", "description" : "shrug"},
{"value" : "（＾－＾）", "description" : "smiling"}
]
```

and `something_else.csv` would be like this
```
value,description
ಠ_ಠ,skeptical
(╯°□°）╯︵ ┻━┻,tableflip
```