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