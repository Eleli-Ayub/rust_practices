pub fn arrays() {
    // Arrays are fixed-size lists of elements of the same type
    let a = [1, 2, 3, 4, 5];
    for x in a.iter(){
        println!("{}", x);
    }
    // Arrays can be initialized with the same value
    let mut b = [0; 5];
    b[2] = 3;
    println!("Array: {:?}", b);
    // Arrays can be nested
    let c = [[1, 2], [3, 4]];
    println!("Array: {:?}", c);
}
