# A VHS tape. See https://github.com/charmbracelet/vhs
# Keyboard path of the mouse example (VHS cannot record mouse input).
Output "examples/vhs/output/mouse.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 600
Set FontSize 22
Set Padding 20
Set PlaybackSpeed 1

Hide
Type "clear && cargo run --release --features crossterm --example mouse"
Enter
Sleep 6s
Show

Sleep 1s
Right
Sleep 0.5s
Right
Sleep 0.8s
Tab
Sleep 0.4s
Type "5"
Sleep 0.8s
Tab
Sleep 0.4s
Type "2"
Sleep 0.8s
Tab
Sleep 0.4s
Right
Sleep 0.4s
Right
Sleep 0.8s
Type "r"
Sleep 1s
Type "r"
Sleep 0.6s
Type "0"
Sleep 1s

Hide
Type "q"
Sleep 0.3s
