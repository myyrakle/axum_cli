<<<<<<< HEAD
use std::path::Path;
=======
use std::{path::PathBuf, str::FromStr};
>>>>>>> 28705d68f528975c0aaecad9f5b03fafaec44b23

use super::zip::ParsedZipData;

pub async fn write_template(template: ParsedZipData, base_path: String) {
<<<<<<< HEAD
    if !Path::new(base_path.as_str()).exists() {
        std::fs::create_dir(&base_path).unwrap();
        println!(">>>>> {} >>> directory created", base_path);
=======
    let base_path = PathBuf::from_str(base_path.as_str()).unwrap();

    if !base_path.exists() {
        std::fs::create_dir(&base_path).unwrap();
        println!(
            ">>>>> {} >>> directory created",
            base_path.to_str().unwrap()
        );
>>>>>>> 28705d68f528975c0aaecad9f5b03fafaec44b23
    }

    for (path, data) in template.into_iter() {
        let path = base_path.join(path).to_str().unwrap().to_owned();

        if data.is_some() {
            std::fs::write(&path, data.unwrap()).unwrap();
            println!(">>>>> {} >>> file created", path);
        } else {
            match std::fs::create_dir(&path) {
                Err(error) => {
                    if error.kind() == std::io::ErrorKind::AlreadyExists {
                        println!(">>>>> {} >>> directory already exists", path);
                    } else {
                        println!(">>>>> {} >>> directory creation failed (error:?)", path);
                    }
                }
                Ok(_) => println!(">>>>> {} >>> directory created", path),
            }
        }
    }

    println!("#### Generation Success ####");
    println!("#### Just Run `cargo run` ####")
}
