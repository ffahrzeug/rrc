pub struct ExecSpec {
    // command to strart service
    pub start: Argv,
    // command to stop service
    pub stop: Argv,
    // command to reload service
    pub reload: Argv,
}

pub struct Argv(Vec<String>);

impl From<String> for Argv {
    fn from(v: String) -> Self {
        let args = v.split_whitespace().map(String::from).collect();
        Argv(args)
    }
}