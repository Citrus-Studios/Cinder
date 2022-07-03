pub enum DefaultingValue<T> {
    Unique(T),
    Default(T),
}

impl<T> DefaultingValue<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Unique(t) => t,
            Self::Default(t) => t,
        }
    }
}
