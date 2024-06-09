pub fn all_loops(){
    let mut count = 0;
    while count <= 10{
        println!("Count: {}", count);
        count += 1;
    }
    
    for n in 1..count {
        if n % 2 == 0{    
            println!("for {}", n);
        }
    }
}
