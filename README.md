# rjrn -> r(ust) journal

Simple command line application for my own private usage to quickly save very short snippets.

Config is saved at home directory by name: `.rjrn.config`

File journal is a simple json file.

# Usage:

```
rjrn --help


rjrn --add // Bootstrap new journal file
rjrn "Quick snippet, like a tweet" // Saved in a journal file

rjrn "Note in work journal" --journal work
alias rjrn-work="rjrn --journal work"
rjrn "Note in work journal"
```

There is some debug statements in the program. If you'd like to see them, please run commands with `--verbose` flag.


# To do:

  - [ ] Trello handler for notes
  - [ ] Refactor how config handles trait for other journal types (right now it's hardcoded file journal)
  - [ ] Add tests for adding notes


# Notes

It was created mostly for learning purposes of rust.

MIT License

**Not actively maintained**
