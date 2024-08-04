use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{self, Read};
use super::formatting::OutputSyntax;


#[derive(Default)]
#[derive(Debug)]
pub struct InitialConfig {
    pub path_or_data: String,
    pub output_syntax: OutputSyntax
}

impl InitialConfig {
    pub fn new(args: Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments passed, Expected format data_to_byte_array.exe <path_to_file or raw bytes> <optional output syntax>");
        }
        
        let mut iter = args.into_iter().skip(1);
        let mut return_data = Self::default();
        return_data.path_or_data = iter.next().unwrap();
        if let Some(data) = iter.next() {
            return_data.output_syntax = OutputSyntax::from_string(&data);
        }
        Ok(return_data)
    }
    

    pub fn get_byte_data(path: String) -> Result<Vec<u8>, Box<dyn Error>> {
        let return_data: Vec<u8>;
        if Path::new(&path).is_file() {
            let buffer = File::open(&path)?;
            return_data = buffer.bytes().into_iter().collect::<Result<Vec<u8>, io::Error>>()?;
        } else {
            return_data = path.into();
        }

        Ok(return_data)
    }
}
impl std::fmt::Display for InitialConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "path or data: {}", self.path_or_data)?;
        writeln!(f, "output syntax: {}", self.output_syntax)
    }
}