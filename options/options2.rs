// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type

        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }
        //this is a better tool than match when failure options are abundent and hard to predict?
    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];
        //make a mutable i8 option vector

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }
        //populate optional_integers with option(i)
        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.

        while let integer = optional_integers.pop().flatten() {
            // this feels unsafe having cursor be mutable
            if cursor >= 1 {
            println!("{} and {}",integer.unwrap(),cursor);
            assert_eq!(integer.unwrap(), cursor);
            cursor -= 1;
            }
            else{
                break
            }
        }
        //Unwrap() seems to work fine but I cant call flatten() twice because
        //optional_integers is not an iterator?

        assert_eq!(cursor, 0);
    }
}
