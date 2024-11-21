use std::fs;

use config::ConfigFile;

pub mod config;
pub mod operations;
pub mod help;

#[derive(Debug)]
struct File {
    name: String,
    content: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    subfolders: Vec<Folder>,
    files: Vec<File>,
}

impl Folder {
    fn new(name: &str) -> Folder {
        Folder {
            name: String::from(name),
            subfolders: Vec::new(),
            files: Vec::new(),
        }
    }

    fn add_subfolder(&mut self, folder: Folder) {
        self.subfolders.push(folder);
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn create(&self, base_path: &str) {
        let path = format!("{}/{}", base_path, self.name);
        fs::create_dir_all(&path).expect("Could not create folder");

        for folder in &self.subfolders {
            folder.create(&path);
        }

        for file in &self.files {
            let file_path = format!("{}/{}", path, file.name);

            fs::write(&file_path, &file.content).expect("Could not create file");
        }
    }
}

pub fn init(base_path: &str) {
    println!("Initializing project...");

    // Create migrations folder
    let mut migrations = Folder::new("");
    let mut pg = Folder::new("pg");
    let mut quest = Folder::new("quest");

    pg.add_subfolder(Folder::new("deploy"));
    pg.add_subfolder(Folder::new("revert"));
    pg.add_subfolder(Folder::new("verify"));

    quest.add_subfolder(Folder::new("deploy"));
    quest.add_subfolder(Folder::new("revert"));
    quest.add_subfolder(Folder::new("verify"));

    pg.add_file(File {
        name: "migrator.plan".to_string(),
        content: "".to_string(),
    });

    quest.add_file(File {
        name: "migrator.plan".to_string(),
        content: "".to_string(),
    });

    migrations.add_subfolder(pg);
    migrations.add_subfolder(quest);

    migrations.add_file(File {
        name: "config.toml".to_string(),
        content: ConfigFile::stub().into(),
    });

    migrations.create(base_path);
}
