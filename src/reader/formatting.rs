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

pub enum OutputSyntax {
    Raw(RawFormatter),
    Cpp(CppFormatter),
    C(CFormatter),
    Rust(RustFormatter),
}

impl Default for OutputSyntax {
    fn default() -> Self {
        OutputSyntax::Raw(RawFormatter)
    }
}

impl OutputSyntax {
    pub fn from_string(target: &String) -> Self {
        match target.to_lowercase().as_str() {
            "cpp" | "c++" | "cxx" => OutputSyntax::Cpp(CppFormatter),
            "c" => OutputSyntax::C(CFormatter),
            "rust" => OutputSyntax::Rust(RustFormatter),
            _ => OutputSyntax::Raw(RawFormatter),
        }
    }
}

impl std::fmt::Display for OutputSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw(_) => write!(f, "Raw"),
            Self::Rust(_) => write!(f, "Rust"),
            Self::Cpp(_) => write!(f, "Cpp"),
            Self::C(_) => write!(f, "C"),
        }
    }
}

impl std::fmt::Debug for OutputSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw(_) => write!(f, "Raw"),
            Self::Rust(_) => write!(f, "Rust"),
            Self::Cpp(_) => write!(f, "Cpp"),
            Self::C(_) => write!(f, "C"),
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
            OutputSyntax::C(_) => CFormatter::format(&self.bytes),
            OutputSyntax::Cpp(_) => CppFormatter::format(&self.bytes),
            OutputSyntax::Rust(_) => RustFormatter::format(&self.bytes),
            _ => RawFormatter::format(&self.bytes),
        }
    }
}
