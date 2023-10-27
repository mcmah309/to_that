//! Declarative compile safe explict type conversion. Useful for chaining.


impl<T> To for T {}
pub trait To: Into<Self> {
    /// Converts self to R
    fn to<R: From<Self>>(self) -> R {
        R::from(self.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct A(i32);

    #[derive(Debug, PartialEq)]
    struct B(i32);

    impl From<A> for B {
        fn from(a: A) -> Self {
            B(a.0)
        }
    }
    #[test]
    fn it_works() {
        let result = A(1).to::<B>();
        assert_eq!(result, B(1));
    }
}
