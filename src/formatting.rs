pub trait FormatProvider {
    fn get_strikethrough(&self, s: &str) -> String;
    fn get_newline(&self) -> String;
}

pub trait Format {
    fn format(&self, fp: &FormatProvider) -> String;
}

pub struct GitterMdFormatProvider;

impl GitterMdFormatProvider {
    pub fn new() -> GitterMdFormatProvider{
        GitterMdFormatProvider
    }
}

impl FormatProvider for GitterMdFormatProvider {
    fn get_strikethrough(&self, s: &str) -> String {
        format!("~~{}~~", s)
    }

    fn get_newline(&self) -> String {
        format!("\n")
    }
}
