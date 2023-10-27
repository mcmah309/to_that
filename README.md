# to_that

Declarative compile safe explict type conversion. Useful for chaining.

### Example
For
```rust
    #[derive(Debug, PartialEq)]
    struct A(i32);

    #[derive(Debug, PartialEq)]
    struct B(i32);

    impl From<A> for B {
        fn from(a: A) -> Self {
            B(a.0)
        }
    }
```
Instead of 
```rust
    fn main() {
        let result = B::from(A(1));
        assert_eq!(result, B(1));
    }
```
You can now do
```rust
    fn main() {
        let result = A(1).to::<B>();
        assert_eq!(result, B(1));
    }
```