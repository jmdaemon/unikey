pub mod tmpl;
pub mod keyboard;
pub mod parse;
pub mod linux;

#[cfg(test)]
pub mod tests {
    use crate::tmpl::format_rendered;
    #[test]
    pub fn format_rendered_should_not_be_none() {
        let result = format_rendered("MyTitle", "MyRenderedContent");
        assert_ne!(Some(result), None);
    }
}
