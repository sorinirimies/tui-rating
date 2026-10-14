# A VHS tape. See https://github.com/charmbracelet/vhs

# for running against the tui-rating git repo https://github.com/sorinirimies/tui-rating
Output "examples/vhs/output/interactive.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 700
Set FontSize 22
Set Padding 20
Set PlaybackSpeed 1

Hide
Type "cargo run --release --example interactive"
Enter
Sleep 5s
Show

Sleep 1s
Down
Sleep 0.3s
Right
Sleep 0.3s
Down
Sleep 0.3s
Right Right
Sleep 0.3s
Down
Sleep 0.3s
Left
Sleep 0.3s
Type "5"
Sleep 1s
Type "t"
Sleep 0.7s
Type "t"
Sleep 1s

Hide
Type "q"
Sleep 0.3s
