use config::config::Config;
use std::process::Command;

pub struct Commander<'a>
{
    config: &'a Config,
}

impl<'a> Commander<'a>
{
    pub fn new(config: &Config) -> Commander
    {
        Commander{config}
    }

    fn do_powershell_command(command: &str) -> Result<String, std::io::Error>
    {
        let output = Command::new("powershell")
            .arg(command)
            .output();

        return match output
        {
            Ok(s) => {
                let v = s.stdout;
                let output_string = String::from_utf8(v).unwrap();
                Ok(output_string)
            }
            Err(e) => { Err(e) }
        }
    }

    fn make_directory(&self)
    {
        let make_dir_command = format!
        (
            "mkdir \"{}\\{}\\{}\"",
            self.config.target_path(),
            self.config.project_name(),
            self.config.data_path(),
        );
        //println!("{}", make_dir_command); // DEBUG
        Commander::do_powershell_command(&make_dir_command).unwrap();
    }

    fn copy_exe_file(&self)
    {
        let debug_or_release = if self.config.is_release_mode() {
            "release"
        } else {
            "debug"
        };

        let exe_file_path = format!
        (
            ".\\target\\{}\\{}.exe",
            debug_or_release,
            self.config.project_name()
        );

        let copy_exe_file_command = format!
        (
            "copy \"{}\" \"{}\\{}\"",
            exe_file_path,
            self.config.target_path(),
            self.config.project_name(),
        );

        println!("{}", copy_exe_file_command); // DEBUG
        Commander::do_powershell_command(&copy_exe_file_command).unwrap();
    }

    fn copy_data(&self)
    {
        let copy_data_command = format!
        {
            "copy \".\\{}\\*\" \"{}\\{}\\{}\"",
            self.config.data_path(),
            self.config.target_path(),
            self.config.project_name(),
            self.config.data_path(),
        };

        println!("{}", copy_data_command); // DEBUG
        Commander::do_powershell_command(&copy_data_command).unwrap();
    }

    fn build_project(&self)
    {
        let build_command = if self.config.is_release_mode() {
            "cargo build --release"
        } else {
            "cargo build"
        };

        println!("{}", build_command); // DEBUG
        Commander::do_powershell_command(build_command).unwrap();
    }

    pub fn execute(&self)
    {
        self.build_project();
        self.make_directory();
        self.copy_exe_file();
        self.copy_data();
    }
}