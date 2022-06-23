pub enum Fallback<T> {
    Unique(T),
    Fallback(T),
}

impl<T> Fallback<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Unique(t) => t,
            Self::Fallback(t) => t,
        }
    }
}
