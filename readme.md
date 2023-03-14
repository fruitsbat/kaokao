# kaokao
a simple emoji picker using rofi

## usage
simply run the binary and a rofi window will pop up!
you do of course need rofi (or a dropin) installed for this to work
setting the bianary to use with `--rofi-binary` is also possible

### copy to clipboard
kaokao does not provide its own copy to clipboard feature,
but you can simply pipe the output to your clipboard manager of choice

on x11 this is `kaokao | xclip -selection clipboard`
or if you believe that wayland is the future you can do `kaokao | wl-copy`

### custom emojis

custom mojis are supported using either json or csv
if no `.json` extension can be found then kaokao will always try to parse them as csv
use your own files `--files something.json something_else.json`

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
ಠ_ಠ,sceptical
(╯°□°）╯︵ ┻━┻,tableflip
```