use super::parse_zip::ParsedZipData;

pub async fn write_template(template: ParsedZipData, base_path: String) {
    for (path, data) in template.into_iter() {
        let path = [base_path.clone(), path].join("/");

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
}
