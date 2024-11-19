#[derive(Debug)]
pub struct LongFlag {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Cli {
    pub short_flags: Vec<String>,
    pub long_flags: Vec<LongFlag>,
    pub args: Vec<String>,
    pub params: Vec<String>,
}

impl Cli {
    pub fn new() -> Cli {
        let params = std::env::args().collect::<Vec<String>>();
        let mut result = Cli {
            short_flags: vec![],
            long_flags: vec![],
            args: vec![],
            params: params.clone(),
        };

        if params.len() < 2 {
            return result;
        }

        let mut is_long_flag = false;
        let mut long_flag_name = "".to_string();

        for i in 1..params.len() {
            let arg = &params[i];
            
            if arg.starts_with("-") {
                if arg.starts_with("--") {
                    is_long_flag = true;
                    long_flag_name = arg.clone();
                } else {
                    let len = arg.len();
                    for j in 1..len {
                        result.short_flags.push(arg.chars().nth(j).unwrap().to_string());
                    }
                }
            } else {
                if is_long_flag {
                    result.long_flags.push(LongFlag {
                        name: long_flag_name.clone(),
                        value: arg.clone(),
                    });
                    is_long_flag = false;
                } else {
                    result.args.push(arg.clone());
                }
            }
        }

        if is_long_flag {
            result.long_flags.push(LongFlag {
                name: long_flag_name.clone(),
                value: "".to_string(),
            });
        }

        result
    }

    pub fn get_long_flag(&self, name: &str) -> Option<&LongFlag> {
        for long_flag in &self.long_flags {
            if long_flag.name == name {
                return Some(long_flag);
            }
        }

        None
    }

    pub fn base_path(&self) -> String {
        let mut base_path = String::from("./db");

        if let Some(flag) = self.get_long_flag("--base-path") {
            base_path = flag.value.clone();
        }

        base_path
    }
}