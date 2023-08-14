// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    // #[test]
    // fn you_have_asserted_eq() {
    //     assert_eq!(88.0, 8.8*10.0)
    // }
    // it only checks the first one for some reason, I'm sure this will become clear.
    #[test]
    fn you_can_assert_eq() {
        assert_eq!("this is a string", "this is another string");
    }
}
