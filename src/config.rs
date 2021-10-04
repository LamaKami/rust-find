pub struct Config{
    pub query: String,
    pub path: String,
    pub is_substring: bool
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str>{
        let path: String;
        let mut substring = true;

        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();

        path = args[2].clone();

        for arg in args.iter().enumerate(){
            if arg.1.eq("-s") && args[arg.0+1].eq("y"){
                substring = true;
            }
            else if arg.1.eq("-s") && args[arg.0+1].eq("n"){
                substring = false;
            }
        }
        Ok(Config{query, path, is_substring: substring })
    }
}

