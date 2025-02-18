fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y = 10; // Modify x using y

    println!("x: {}", x); // x is now 10

    //Or if you need to modify later, create new reference
    {
        let z = &mut x; // z is a mutable reference to x
        *z = 20; // Modify x using z
    }
    println!("x: {}", x); // x is now 20
    //or clone the value
    let mut x2 = x.clone();
    x2 = 30;
    println!("x2: {}", x2); // x2 is now 30. original x is not affected

}