
pub enum CinderResult {
    Ok(i32),
    Err(i32),
}

impl CinderResult {
    pub fn new(result: i32) -> Self {
        match result {
            0 => CinderResult::Ok(result),
            _ => CinderResult::Err(result),
        }
    }
    pub fn is_ok(&self) -> bool {
        match self {
            CinderResult::Ok(_) => true,
            _ => false,
        }
    }
    pub fn is_err(&self) -> bool {
        match self {
            CinderResult::Err(_) => true,
            _ => false,
        }
    }
    pub fn unwrap(self) -> i32 {
        match self {
            CinderResult::Ok(value) => value,
            CinderResult::Err(value) => panic!("{}", value),
        }
    }
    pub fn unwrap_or(self, err_value: i32) -> i32 {
        match self {
            CinderResult::Ok(value) => value,
            CinderResult::Err(_) => err_value,
        }
    }
    pub fn unwrap_or_else(self, err_func: impl FnOnce() -> i32) -> i32 {
        match self {
            CinderResult::Ok(value) => value,
            CinderResult::Err(_) => err_func(),
        }
    }
}