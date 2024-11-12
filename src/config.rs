use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct TransformConfig {
    pub attribute_name: Option<String>,
} 