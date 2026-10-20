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
Sleep 5s
Show

# pages auto-advance (2.6s each, animation page 4.2s) and every value moves
Sleep 20.5s

Hide
Type "q"
Sleep 0.3s
