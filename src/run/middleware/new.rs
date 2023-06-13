use std::io::Write;
use std::path::PathBuf;

use crate::{
    constants::TEMPLATE_URL,
    utils::{
        write::write_template,
        zip::{get_data_pair_from_zip, get_zip},
    },
};

use convert_case::{Case, Casing};

pub async fn run_new_middleware(middleware_name: String, template_name: String) {
    let snake_case = middleware_name.to_case(Case::Snake);
    let pascal_case = middleware_name.to_case(Case::Pascal);
    let kebab_case = middleware_name.to_case(Case::Kebab);

    let template_name = "middleware_default";

    let zip_data = get_zip(TEMPLATE_URL).await.unwrap();
    let parsed_zip = get_data_pair_from_zip(template_name, zip_data)
        .await
        .unwrap();

    let mapped = parsed_zip
        .into_iter()
        .map(|(path, data)| {
            let path = path.replace("#name#", snake_case.as_str());

            if let Some(data) = data {
                if let Ok(text) = String::from_utf8(data.clone()) {
                    let text = text.replace("#name#", snake_case.as_str());
                    let text = text.replace("#Name#", pascal_case.as_str());
                    let data: Vec<u8> = text.bytes().collect();

                    (path, Some(data))
                } else {
                    (path, Some(data))
                }
            } else {
                (path, data)
            }
        })
        .collect::<Vec<_>>();

    let base_path = PathBuf::new().join("src").join("middlewares");

    if !base_path.exists() {
        panic!("src/middlewares directory not found")
    }

    write_template(mapped, base_path).await;

    let root_router_code = std::fs::read_to_string(&root_router_path).unwrap();
    let new_line = if root_router_code.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };

    {
        let middlewares_root_mod_path = base_path.join("mod.rs");

        if !middlewares_root_mod_path.exists() {
            panic!("src/middlewares/mod.rs not found")
        }

        let import_code = format!(r#"{new_line}pub(crate) mod {snake_case};"#);

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&root_module_path)
            .unwrap();
        file.write_all(import_code.as_bytes()).unwrap();

        // println!(">>>>> {:?} >>> file updated", root_module_path);
    }
}
