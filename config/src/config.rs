pub struct Config
{
    current_path: String,
    project_name: String,
    target_path: String,
    data_path: String,
    is_release_mode: bool,
}

impl Config
{
    fn get_current_path() -> String
    {
        let current_path = std::env::current_dir().unwrap();
        let current_path = current_path.to_str().unwrap();
        return String::from(current_path);
    }

    fn get_catalog_name(path: &String) -> String
    {
        String::from(&path[path.rfind("\\").unwrap() + 1..])
    }

    pub fn new(arguments: Vec<String>) -> Config
    {
        let current_path = Config::get_current_path();
        let project_name = Config::get_catalog_name(&current_path);

        let target_path = arguments.get(1).expect("Could not read target path.");
        let target_path = String::from(target_path);

        let data_path = match arguments.get(2) {
            None => String::from("data"),
            Some(path) => String::from(path),
        };

        let is_release_mode = match arguments.get(3)
        {
            Some(arg) if arg.to_lowercase() == "release" => true,
            _ => false,
        };

        Config
        {
            current_path, project_name, target_path, data_path, is_release_mode
        }
    }

    pub fn current_path(&self) -> &String
    {
        &self.current_path
    }

    pub fn project_name(&self) -> &String
    {
        &self.project_name
    }

    pub fn target_path(&self) -> &String
    {
        &self.target_path
    }

    pub fn data_path(&self) -> &String
    {
        &self.data_path
    }

    pub fn is_release_mode(&self) -> bool
    {
        self.is_release_mode
    }
}