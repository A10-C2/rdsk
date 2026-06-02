# rdsk
A terminal based disk space analyzer written in rsut. Scans user profiles under `C:\Users` and breaks down disk usage by category.
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

### Note
- Symlinks and junctions are skipped to avoid weird shit
- Only for windows, only for user profiles
- Make sure to have rust installed