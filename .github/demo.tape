# Run `vhs .github/demo.tape` to update the demo gif

Output .github/demo.gif

Set Shell "bash"
Set FontSize 14
Set Width 1080
Set Height 700

Type "import-holmes --path example/new --language typescript --sort occurrences" Sleep 500ms Enter

Sleep 5s

Type "clear" Sleep 500ms  Enter

Sleep 1s

Type "import-holmes --path example/kotlin --language kotlin --sort referenced" Sleep 500ms Enter

Sleep 5s

Type "clear" Sleep 500ms  Enter

Sleep 1s

Type "import-holmes --path example/new --language typescript --sort occurrences -o json | jq" Sleep 500ms Enter

Sleep 5s

Type "clear" Sleep 500ms  Enter

Sleep 1s
