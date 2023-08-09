// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


struct Wrapper<Gen> {
    value: Gen,
}

//the wrapper needs the generic type name in the signature to be in scope.

impl<Gen> Wrapper<Gen> {
    pub fn new(value: Gen) -> Self {
        Wrapper { value }
    }
}
//impl defines on the type, so it requires generic type names. 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
