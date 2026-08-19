use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ExecSpec {
    // command to strart service
    pub start: Argv,
    // command to stop service
    pub stop: Option<Argv>,
    // command to reload service
    pub reload: Option<Argv>,

    // todo: cpu/memory/io/network limits
}

#[derive(Serialize, Deserialize)]
pub struct Argv(Vec<String>);

impl From<String> for Argv {
    fn from(v: String) -> Self {
        let args = v.split_whitespace().map(String::from).collect();
        Argv(args)
    }
}