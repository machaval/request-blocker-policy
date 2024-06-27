use serde::Deserialize;
#[derive(Deserialize, Clone, Debug)]
pub struct All0Config {
    #[serde(alias = "headerName")]
    pub header_name: String,
    #[serde(alias = "headerValue")]
    pub header_value: String,
}
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(alias = "all")]
    pub all: Vec<All0Config>,
}
