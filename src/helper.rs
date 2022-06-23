pub enum OptionOr<T> {
    Some(T),
    None(T),
}

impl<T> OptionOr<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Some(t) => t,
            Self::None(t) => t,
        }
    }
}
