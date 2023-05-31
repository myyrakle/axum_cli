use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CargoToml {
    package: CargoPackage,
    dependencies: toml::value::Value,
    features: Option<toml::value::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CargoPackage {
    name: String,
    version: toml::value::Value,
    authors: toml::value::Value,
    edition: toml::value::Value,
}

impl CargoPackage {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl CargoToml {
    pub fn set_name(&mut self, name: String) {
        self.package.set_name(name);
    }
}

pub fn edit_cargo_toml(
    project_name: String,
    source: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut cargo_toml: CargoToml = toml::from_str(&source)?;
    cargo_toml.set_name(project_name);
    Ok(toml::to_string(&cargo_toml)?)
}
