pub trait Validation {
    fn validate(&self) -> Result<(), &str>;
}
