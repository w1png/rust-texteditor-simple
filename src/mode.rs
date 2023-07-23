pub enum Mode {
    Normal,
    Input,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Input => write!(f, "INPUT"),
        }
    }
}
