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

pub async fn run_new_router(router_name: String, template_name: String) {
    let snake_case = router_name.to_case(Case::Snake);
    let pascal_case = router_name.to_case(Case::Pascal);
    let kebab_case = router_name.to_case(Case::Kebab);

    let template_name = match template_name.as_str() {
        "crud" => "new_route_with_crud",
        _ => panic!("template name not found"),
    };

    let zip_data = get_zip(TEMPLATE_URL).await.unwrap();
    let parsed_zip = get_data_pair_from_zip(template_name, zip_data)
        .await
        .unwrap();

    let nest_code = format!(
        r#"        .nest("/{kebab_case}", crate::routes::{snake_case}::router::get_router().await)"#
    );

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

    let base_path = PathBuf::new().join("src").join("routes");

    if !base_path.exists() {
        panic!("src/routes directory not found")
    }

    let base_path = base_path.join(&snake_case).to_str().unwrap().to_owned();

    write_template(mapped, base_path).await;

    let root_router_path = PathBuf::new()
        .join("src")
        .join("routes")
        .join("root")
        .join("router.rs");

    if !root_router_path.exists() {
        panic!("src/routes/root/router.rs file not found")
    }

    let root_router_code = std::fs::read_to_string(&root_router_path).unwrap();
    let new_line = if root_router_code.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };

    {
        let root_router_code: String = root_router_code
            .split(new_line)
            .map(|line| {
                if line.contains("// Append the new route here.") {
                    format!("{nest_code}{new_line}{line}")
                } else {
                    line.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join(new_line);

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&root_router_path)
            .unwrap();
        file.write_all(root_router_code.as_bytes()).unwrap();
        // println!(">>>>> {:?} >>> file updated", root_router_path);
    }

    {
        let root_module_path = PathBuf::new().join("src").join("routes").join("mod.rs");

        if !root_module_path.exists() {
            panic!("src/routes/mod.rs file not found")
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
