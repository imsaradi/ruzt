fn shorter<'l>(x: &'l str, y: &'l str) -> &'l str {
    if x.len() < y.len() {
        x
    } else {
        y
        }
    }

        fn slice_first_five(s: &String) -> &str {
            //    fn slice_first_five(s: &String) -> &str {
        &s[..5]
    } //  Now slice_first_five is missing its lifetime annotation — 
        //try compiling it and watch Rust stop you with its classic "missing lifetime specifier" error

    fn first_word<'a>(s: &'a String) -> &'a str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }


fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let result = shorter(&s1, &s2);
    println!("The shorter string is: {}", result);
    let word = String::from("Narutomaki");
    let slice = slice_first_five(&word);
    println!("The slice is: {}", slice); // Should print: "Naru"
    let phrase = String::from("Rust is amazing");
    let word = first_word(&phrase);
    println!("The first word is: {}", word); // Should print: "Rust"
}