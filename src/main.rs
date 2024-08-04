use reader::config;
use reader::formatting;
use std::env;
use std::error::Error;
use std::process;
pub mod reader;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    let initial_config = config::InitialConfig::new(args)
        .unwrap_or_else(
            |err| {
                println!("Failed to parse CLI args, {}", err);
                process::exit(-1);
            });
    println!("{}", initial_config);
    let formatting_data = formatting::FormattingData::new(
        config::InitialConfig::get_byte_data(initial_config.path_or_data)?,
        initial_config.output_syntax
    );
    println!("Hex dump,\n{}", formatting_data.write_to_string());
    
    Ok(())
}
