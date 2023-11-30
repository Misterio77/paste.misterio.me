use crate::error::ServerError;
use syntect::parsing::SyntaxSet as SyntectSyntaxSet;
use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    util::LinesWithEndings,
};

pub struct SyntaxSet(SyntectSyntaxSet);

impl SyntaxSet {
    pub fn new(bin: &[u8]) -> Self {
        let loaded: SyntectSyntaxSet = syntect::dumps::from_binary(bin);
        let mut builder = loaded.into_builder();
        builder.add_plain_text_syntax();
        let inner = builder.build();
        Self(inner)
    }

    pub fn highlight(&self, extension: Option<&str>, content: &str) -> Result<String, ServerError> {
        let ext = extension.unwrap_or("txt");
        let syntax = self
            .0
            .find_syntax_by_extension(ext)
            .unwrap_or_else(|| self.0.find_syntax_plain_text());

        let mut generator =
            ClassedHTMLGenerator::new_with_class_style(syntax, &self.0, ClassStyle::Spaced);

        for line in LinesWithEndings::from(content) {
            generator.parse_html_for_line_which_includes_newline(line)?;
        }
        Ok(generator.finalize())
    }
}
