

// ❌ This will cause a compilation error (Dangling reference example)
fn main() {
    // Uncomment below to see Rust protect you:
     let r = dangle();
     println!("Dangling reference: {}", r);

    // ✅ Correct way: Move ownership instead
    let r = no_dangle();
    println!("No dangling reference: {}", r);
}

// This function tries to return a reference to a local String - BAD
 fn dangle() -> &String {
     let s = String::from("oops");
     &s
 }

// Correct version: return the String itself
fn no_dangle() -> String {
    let s = String::from("all good");
    s
}