use std::fs::File;
use std::path::Path;

const CONFIGURATION_FILE_PATH: &'static str = ".sshc";

fn configuration_file_exists() -> bool {
    let file_exists: bool = Path::new(CONFIGURATION_FILE_PATH).exists();

    return file_exists;
}

fn create_configuration_file() -> File {
    let configuration_file_path: &Path = Path::new(CONFIGURATION_FILE_PATH);

    let configuration_file: File = match File::create(configuration_file_path) {
        Err(error) => panic!("Could not create configuration file: {}", error),
        Ok(file) => file,
    };

    return configuration_file;
}

fn open_configuration_file() -> File {
    let configuration_file_exists: bool = configuration_file_exists();

    let configuration_file_path = Path::new(CONFIGURATION_FILE_PATH);
    let mut configuration_file: File;

    if !configuration_file_exists {
        configuration_file = create_configuration_file();
    } else {
        configuration_file = match std::fs::File::open(configuration_file_path) {
            Err(error) => panic!("Could not open configuration file: {}", error),
            Ok(file) => file,
        }
    }

    return configuration_file;
}

pub fn read_configuration_file() {
    let configuration_file: File = open_configuration_file();
}
