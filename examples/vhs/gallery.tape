# A VHS tape. See https://github.com/charmbracelet/vhs
# Tour of every design (cycles through the gallery pages).
Output "examples/vhs/output/gallery.gif"
Set Theme "Aardvark Blue"
Set Width 1300
Set Height 760
Set FontSize 22
Set Padding 20
Set PlaybackSpeed 1

Hide
Type "clear && cargo run --release --example gallery"
Enter
Sleep 6s
Show

Sleep 2.5s
Right
Sleep 2.5s
Right
Sleep 3s
Right
Sleep 3s
Right
# pause auto-advance so the animation page can play out
Type " "
Sleep 6s

Hide
Type "q"
Sleep 0.3s
