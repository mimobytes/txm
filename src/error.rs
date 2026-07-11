use std::ops::Range;

#[derive(Debug, Clone, Default, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct ParseError(pub String);
 
impl ParseError {
    pub fn from_range(range: Range<usize>) -> Self {
        Self(format!("Invalid token at byte {}", range.start))
    }
}
