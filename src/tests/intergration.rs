#[cfg(test)]
mod test {
    use crate::color;

    #[test]
    pub fn color_check() {
        assert_eq!(
            color::mccolor_esc("&dTESTING", "&"),
            format!("{}TESTING", color::Text::new().bright_magenta().build()),
        )
    }
}
