# A VHS tape. See https://github.com/charmbracelet/vhs

# for running against the tui-rating git repo https://github.com/sorinirimies/tui-rating
Output "examples/vhs/output/symbols.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 600
Set FontSize 22
Set Padding 20
Set PlaybackSpeed 1

Hide
Type "clear && cargo run --release --example symbols"
Enter
Sleep 5s
Show

Sleep 1s
Right
Sleep 0.4s
Right
Sleep 0.4s
Left Left Left
Sleep 0.6s
Right Right
Sleep 1s

Hide
Type "q"
Sleep 0.3s
