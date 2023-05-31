use crate::{
    constants::TEMPLATE_URL,
    utils::{
        toml::edit_cargo_toml,
        write::write_template,
        zip::{get_data_pair_from_zip, get_zip},
    },
};

pub async fn run_init(project_name: String, template_name: String) {
    let zip_data = get_zip(TEMPLATE_URL).await.unwrap();
    let parsed_zip = get_data_pair_from_zip(template_name.as_str(), zip_data)
        .await
        .unwrap();

    let mapped = parsed_zip
        .into_iter()
        .map(|(path, data)| {
            if path.split("/").last().unwrap() == "Cargo.toml" {
                if let Some(data) = data {
                    let cargo_toml = String::from_utf8(data).unwrap();
                    let cargo_toml = edit_cargo_toml(project_name.clone(), cargo_toml).unwrap();
                    let data: Vec<u8> = cargo_toml.bytes().collect();

                    (path, Some(data))
                } else {
                    (path, data)
                }
            } else {
                (path, data)
            }
        })
        .collect::<Vec<_>>();

    write_template(mapped, ".".into()).await;
}
