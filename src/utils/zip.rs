use std::io::Cursor;
use zip::ZipArchive;

pub type ZipData = ZipArchive<Cursor<Vec<u8>>>;

// 템플릿 소스코드를 zip으로 획득
pub async fn get_zip(url: &str) -> Result<ZipData, Box<dyn std::error::Error>> {
    let bytes: Vec<u8> = reqwest::get(url)
        .await?
        .bytes()
        .await?
        .into_iter()
        .collect();
    let cursor = Cursor::new(bytes);

    match zip::ZipArchive::new(cursor) {
        Err(error) => Err(Box::new(error)),
        Ok(zip) => Ok(zip),
    }
}

use std::borrow::ToOwned;
use std::io::Read;

pub type ParsedZipData = Vec<(String, Option<Vec<u8>>)>;

// zip 데이터를 가공해서 (파일명, 데이터) 쌍 획득
pub async fn get_data_pair_from_zip(
    target_template_name: &str,
    mut zip: ZipData,
) -> Result<ParsedZipData, Box<dyn std::error::Error>> {
    let mut file_list = vec![];

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;

        let template_name = file.name().split("/").nth(1).map(ToOwned::to_owned);
        let is_template = file.name().split("/").nth(2).is_some();

        let mut split = file.name().split("/");
        split.next();
        split.next();
        split.next();
        let path: Vec<&str> = split.collect();
        let path = path.join("/");

        let data = if file.is_file() {
            let data: Vec<u8> = file
                .bytes()
                .filter(|e| e.is_ok())
                .map(|e| e.unwrap())
                .collect();

            Some(data)
        } else {
            None
        };

        let file_value = (path, data);

        if is_template {
            if let Some(template_name) = template_name {
                if target_template_name == template_name {
                    file_list.push(file_value);
                }
            }
        }
    }

    Ok(file_list)
}
