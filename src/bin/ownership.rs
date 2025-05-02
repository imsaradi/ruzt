fn main() {
    // Step 1: Create a new String
    let sweet = String::from("Ferrero Rocher");

    // Step 2: Borrow immutably (multiple immutable borrows allowed)
    let borrow1 = &sweet;
    let borrow2 = &sweet;

    // Step 3: Print both immutable borrows
    println!("First bite: {}", borrow1);
    println!("Second bite: {}", borrow2);

    // Step 4: Move ownership to a new variable (after all borrows are done)
    let yummy = sweet;

    // Step 5: Print the new owner
    println!("Final snack: {}", yummy);


    // Create a mutable String
        let mut hot = String::from("Rocher");
    
        // Mutable borrow
        let snack = &mut hot;
    
        // Modify the borrowed value
        snack.push_str(" - Extra Chocolate");
    
        // Cannot use `hot` directly now because `snack` holds it
        println!("Yummy snack: {}", snack);
    
        // After this, `snack` ends, `sweet` can be used again
}