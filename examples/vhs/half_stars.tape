# A VHS tape. See https://github.com/charmbracelet/vhs

# for running against the tui-rating git repo https://github.com/sorinirimies/tui-rating
Output "examples/vhs/output/half_stars.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 700
Set FontSize 22
Set Padding 20
Set PlaybackSpeed 1

Hide
Type "cargo run --release --example half_stars"
Enter
Sleep 5s
Show

Sleep 1s
Right
Sleep 0.4s
Right
Sleep 0.4s
Right
Sleep 0.4s
Left
Sleep 0.4s
Type "t"
Sleep 0.6s
Right
Sleep 1s

Hide
Type "q"
Sleep 0.3s
