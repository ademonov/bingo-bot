pub trait FormatProvider {
    fn get_strikethrough(&self, s: &str) -> String;
}

pub trait Format {
    fn format(&self, fp: &FormatProvider) -> String;
}

pub struct GitterMdFormatProvider;

impl FormatProvider for GitterMdFormatProvider {
     fn get_strikethrough(&self, s: &str) -> String {
         format!("~~{}~~", s)
     }
}