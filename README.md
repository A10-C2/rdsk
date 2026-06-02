# rdsk
A terminal based disk space analyzer written in rust. Scans user profiles under `C:\Users` and breaks down disk usage by category.
- Total Size
- Appdata local
- Appdata roaming
- Appdata temp
- Teams cache
- Documents
- Downloads
- Desktop
- Other

## Usage
Run as admin with full access to profiles.
`cargo run --release`

### Notes
- Symlinks and junctions are skipped to avoid weird shit
- Only for windows, only for user profiles
- Make sure to have [rust installed](https://rust-lang.org/tools/install/)

Also these: 
- [Ratatui](https://docs.rs/ratatui/latest/ratatui/index.html)
- [Crossterm](https://docs.rs/crossterm/latest/crossterm/index.html)
- [Learn this](https://doc.rust-lang.org/std/sync/mpsc/index.html)

#### TODO
- [ ] add color
- [ ] split main veiw into two instead of two seperate windows
- [ ] read up on std::sync
- [ ] list selection
- [ ] detailed view
- [ ] font?
- [ ] font size?
- [ ] terminal size & pos?

#### Maybe
- [ ] full explorer
- [ ] optimize threading
