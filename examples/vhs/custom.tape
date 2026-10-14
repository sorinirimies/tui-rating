# A VHS tape. See https://github.com/charmbracelet/vhs

# for running against the tui-rating git repo https://github.com/sorinirimies/tui-rating
Output "examples/vhs/output/custom.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 700
Set FontSize 22
Set Padding 20
Set PlaybackSpeed 1

Hide
Type "clear && cargo run --release --example custom"
Enter
Sleep 5s
Show

Sleep 1s
Left
Sleep 0.3s
Left
Sleep 0.3s
Right Right Right
Sleep 0.4s
Right
Sleep 1s

Hide
Type "q"
Sleep 0.3s
