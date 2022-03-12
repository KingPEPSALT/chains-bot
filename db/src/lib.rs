#[macro_use]
pub mod channel;
pub mod guild;
pub mod member;
pub mod warn;
pub use sea_orm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
