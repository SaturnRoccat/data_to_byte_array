pub trait ArrayFormatter {
    fn format(vec: &Vec<u8>) -> String;
}

pub struct RawFormatter;
pub struct CppFormatter;
pub struct CFormatter;
pub struct RustFormatter;

fn raw_format(vec: &Vec<u8>) -> String {
    vec.iter()
        .map(|val| format!("{:#02x}", *val))
        .collect::<Vec<String>>()
        .join(", ")
}

impl ArrayFormatter for RawFormatter {
    fn format(vec: &Vec<u8>) -> String {
        raw_format(vec)
    }
}

impl ArrayFormatter for CppFormatter {
    fn format(vec: &Vec<u8>) -> String {
        format!(
            "std::array<uint8_t, {}> raw_hex = {{ {} }};",
            vec.len(),
            raw_format(vec)
        )
    }
}

impl ArrayFormatter for CFormatter {
    fn format(vec: &Vec<u8>) -> String {
        format!("unsigned char[] raw_hex = {{ {} }};", raw_format(vec))
    }
}

impl ArrayFormatter for RustFormatter {
    fn format(vec: &Vec<u8>) -> String {
        format!("[u8: {}] = [{}]", vec.len(), raw_format(vec))
    }
}

#[derive(Default)]
pub enum OutputSyntax {
    #[default]
    Raw,
    Cpp,
    C,
    Rust,
}

impl OutputSyntax {
    pub fn from_string(target: &String) -> Self {
        match target.to_lowercase().as_str() {
            "cpp" | "c++" | "cxx" => OutputSyntax::Cpp,
            "c" => OutputSyntax::C,
            "rust" => OutputSyntax::Rust,
            _ => OutputSyntax::Raw,
        }
    }
}

impl std::fmt::Display for OutputSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw => write!(f, "Raw"),
            Self::Rust => write!(f, "Rust"),
            Self::Cpp => write!(f, "Cpp"),
            Self::C => write!(f, "C"),
        }
    }
}

impl std::fmt::Debug for OutputSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw => write!(f, "Raw"),
            Self::Rust => write!(f, "Rust"),
            Self::Cpp => write!(f, "Cpp"),
            Self::C => write!(f, "C"),
        }
    }
}

#[derive(Debug)]
pub struct FormattingData {
    bytes: Vec<u8>,
    syntax: OutputSyntax,
}

impl FormattingData {
    pub fn new(bytes: Vec<u8>, syntax: OutputSyntax) -> Self {
        FormattingData {
            bytes: bytes,
            syntax: syntax,
        }
    }

    pub fn write_to_string(self) -> String {
        match self.syntax {
            OutputSyntax::C => CFormatter::format(&self.bytes),
            OutputSyntax::Cpp => CppFormatter::format(&self.bytes),
            OutputSyntax::Rust => RustFormatter::format(&self.bytes),
            _ => RawFormatter::format(&self.bytes),
        }
    }
}
