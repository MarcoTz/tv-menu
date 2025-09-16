use std::{collections::HashMap, path::PathBuf};

pub struct ConfigSection {
    pub(crate) values: HashMap<String, String>,
}

pub struct ConfigContents {
    pub(crate) sections: HashMap<String, ConfigSection>,
    pub(crate) path: PathBuf,
}
