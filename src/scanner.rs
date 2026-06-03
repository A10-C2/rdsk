//! Handles all filesystem scanning logic
//! Provides directory size calculation and Windows users profile scanning.
use std::fs;
use std::path::Path;

pub trait Listable {
    fn name(&self) -> String;
    fn size(&self) -> u64;
}

#[derive(Debug)]
pub struct DirectoryEntry {
    pub name: String,
    pub total_size: u64,
}

impl DirectoryEntry {
    pub fn new(name: String, total_size: u64) -> Self {
        DirectoryEntry { name, total_size }
    }
}

impl Listable for DirectoryEntry {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn size(&self) -> u64 {
        self.total_size
    }
}

#[derive(Debug)]
pub struct UserProfile {
    pub username: String,
    pub total_size: u64,
    pub appdata_local: u64,
    pub appdata_roaming: u64,
    pub appdata_local_temp: u64,
    pub teams_cache: u64,
    pub documents: u64,
    pub desktop: u64,
    pub downloads: u64,
    pub onedrive: u64,
    pub other: u64,
}

impl UserProfile {
    pub fn new(filename: String) -> Self {
        UserProfile {
            username: filename,
            total_size: 0,
            appdata_local: 0,
            appdata_roaming: 0,
            appdata_local_temp: 0,
            teams_cache: 0,
            documents: 0,
            desktop: 0,
            downloads: 0,
            onedrive: 0,
            other: 0,
        }
    }
}

impl Listable for UserProfile {
    fn name(&self) -> String {
        self.username.clone()
    }

    fn size(&self) -> u64 {
        self.total_size
    }
}

/// Scans given directory and returns total size in bytes. Symlinks are skipped during the scan. If a directory is found, recursivly scan it.
///  The size in bytes of all files inside of the directory are added together and returned.
///
/// # Arguments
/// * `path` - Root path to scan. Accpets any std::Path
///
/// # Errors
/// Will return `0` if fs::read_dir() fails.
pub fn scan_directory(path: &Path) -> u64 {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return 0,
    };

    let mut total = 0u64;

    for entry in entries.flatten() {
        let metadata = match entry.path().symlink_metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        if metadata.is_symlink() {
            continue;
        }

        if metadata.is_file() {
            total += metadata.len();
        } else if metadata.is_dir() {
            total += scan_directory(&entry.path());
        }
    }

    total
}

/// Scans C:\Users and builds a UserProfile for each user directory,
/// calculating sizes for common folders like AppData, Downloads, and OneDrive.
/// `C:\Users\` is hardcoded in when building the full path.
///
/// # Arguments
/// * `path` - Root path to scan, typically `C:\Users`, but will accept any std::Path
///
/// # Errors
/// Returns an error if the root path cannot be read.
pub fn scan_users(path: &Path) -> Result<Vec<UserProfile>, Box<dyn std::error::Error>> {
    let entries = fs::read_dir(path)?;
    let mut users: Vec<UserProfile> = vec![];
    let base_path = Path::new(r"C:\Users\");
    for entry in entries {
        let filename = entry.unwrap();
        let name = filename.file_name().into_string().unwrap();
        let is_dir = filename.file_type()?.is_dir();

        let mut user = UserProfile::new(name);

        // println!("User: {:#?}", &user);
        let usr_path = Path::new(&user.username);
        let full_path = base_path.join(usr_path);

        // Total Size
        let total_size = scan_directory(&full_path);
        user.total_size = total_size;

        if !is_dir {
            continue;
        }

        // appdata_local
        let appdata_local_path = full_path.join(Path::new(r"AppData\Local\"));
        user.appdata_local = scan_directory(&appdata_local_path);

        // appdata_roaming
        let appdata_roaming_path = full_path.join(Path::new(r"AppData\Roaming\"));
        user.appdata_roaming = scan_directory(&appdata_roaming_path);

        // appdata_temp
        let appdata_temp = full_path.join(Path::new(r"AppData\Local\Temp"));
        user.appdata_local_temp = scan_directory(&appdata_temp);

        // Teams Cache
        let appdata_teams_cache = full_path.join(Path::new(r"AppData\Local\Microsoft\Teams\Cache"));
        user.teams_cache = scan_directory(&appdata_teams_cache);

        // Documents
        let appdata_temp = full_path.join(Path::new(r"Documents"));
        user.documents = scan_directory(&appdata_temp);

        // Downloads
        let downloads = full_path.join(Path::new(r"Downloads"));
        user.downloads = scan_directory(&downloads);

        // Desktop
        let appdata_temp = full_path.join(Path::new(r"Desktop"));
        user.desktop = scan_directory(&appdata_temp);

        // Onedrive
        let onedrive = full_path.join(Path::new(r"Onedrive\"));
        user.onedrive = scan_directory(&onedrive);

        // Other
        let other_data = user.total_size
            - user.appdata_local
            - user.appdata_roaming
            - user.appdata_local_temp
            - user.teams_cache
            - user.documents
            - user.downloads
            - user.desktop
            - user.onedrive;
        user.other = other_data;

        users.push(user);
    }

    // println!("{:#?}", &users);
    Ok(users)
}
