use config::config::Config;
use commander::commander::Commander;

fn main()
{
    let args: Vec<String> = std::env::args().into_iter().collect();
    let config = Config::new(args);

    let commander = Commander::new(&config);
    commander.execute();
}