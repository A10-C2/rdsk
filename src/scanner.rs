use std::fs;
use std::path::Path;

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
            other: 0,
        }
    }
}

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

        // Other
        let other_data = user.total_size
            - user.appdata_local
            - user.appdata_roaming
            - user.appdata_local_temp
            - user.teams_cache
            - user.documents
            - user.downloads
            - user.desktop;
        user.other = other_data;

        users.push(user);
    }

    // println!("{:#?}", &users);
    Ok(users)
}
