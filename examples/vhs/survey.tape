# A VHS tape. See https://github.com/charmbracelet/vhs
# Keyboard-driven survey form (RatingForm, NPS, Likert, faces, range, slider, thumbs).
Output "examples/vhs/output/survey.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 760
Set FontSize 20
Set Padding 20
Set PlaybackSpeed 1

Hide
Type "clear && cargo run --release --features crossterm --example survey"
Enter
Sleep 5s
Show

Sleep 0.5s
# criteria
Right Right Right
Sleep 0.2s
Down
Right Right Right Right Right Right Right Right Right
Sleep 0.2s
Down
Right Right Right Right Right Right
Sleep 0.4s
Type "]"
# NPS
Type "9"
Sleep 0.5s
Type "]"
# likert
Right Right Right Right
Sleep 0.5s
Type "]"
# faces
Type "4"
Sleep 0.5s
Type "]"
# range
Right
Sleep 0.2s
Type " "
Left Left
Sleep 0.5s
Type "]"
# slider
Right Right Right Right Right Right Right Right Right Right
Sleep 0.3s
Enter
Type "4.2"
Sleep 0.4s
Enter
Sleep 0.5s
Type "]"
# thumbs
Up
Sleep 1.2s

Hide
Type "q"
Sleep 0.3s
