use std::{fs::File, io::Read};

pub fn file_contents(file_path: &str) -> String {
    let mut file =
        File::open(file_path).expect(format!("Unable to open: {:?}", file_path).as_str());

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    data
}
