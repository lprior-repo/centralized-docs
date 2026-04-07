#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub(crate) mod support {
    use super::super::types::*;

    #[allow(dead_code)]
    pub fn make_heading(level: u32, text: &str) -> Heading {
        Heading {
            level,
            text: text.to_string(),
            line: 0,
        }
    }
}
