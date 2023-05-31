use std::io::Cursor;
use zip::ZipArchive;

pub type ZipData = ZipArchive<Cursor<Vec<u8>>>;

// 템플릿 소스코드를 zip으로 획득
async fn get_zip(url: &str) -> Result<ZipData, Box<dyn std::error::Error>> {
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
