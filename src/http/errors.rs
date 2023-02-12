pub enum RequestError {
    MissingChar(char),
}
impl std::fmt::Debug for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingChar(c) => write!(f, "Missing character: {}", c),
        }
    }
}
