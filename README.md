# rdsk
A terminal based disk space analyzer written in rust. Scans user profiles under `C:\Users` and breaks down disk usage by category. Pressing `Tab` will put the app into Explorer Mode, allowing
for file tree navigation.

#### User Profile 
- Username
- Total Size
- Appdata local
- Appdata roaming
- Appdata temp
- Teams cache
- Onedrive (Generic Onedrive, not sharpoint/enterprise)
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

#### Explorer Mode
- Explorer uses lazy loading, but the spinner isn't hooked up to to show when data is getting loaded (yet) *NOT A BUG*
- Launches explorer mode wherever the .exe is located

Also these: 
- [Ratatui](https://docs.rs/ratatui/latest/ratatui/index.html)
- [Crossterm](https://docs.rs/crossterm/latest/crossterm/index.html)
- [Learn this](https://doc.rust-lang.org/std/sync/mpsc/index.html)

#### TODO
- [x] add color
- [x] split main veiw into two instead of two seperate windows
- [x] read up on std::sync
- [x] list selection
- [x] detailed view
- [ ] font?
- [ ] font size?
- [ ] terminal size & pos?

#### Maybe
- [x] full explorer
- [ ] optimize threading
