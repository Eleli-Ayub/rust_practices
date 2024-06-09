use std::io;

pub fn user_input(){
    println!("enter the first name");
    let mut first_name = String::new();
    io::stdin().read_line(&mut first_name).expect("Failed to read user input");
    let mut age_input = String::new();
    println!("enter your age");
    io::stdin().read_line(&mut age_input).expect("Failed to read age input");
    let age_int: f64 = age_input.trim().parse().expect("Failed to parse the entered age into number");
       
    println!("Hello {} your age is {}", first_name, age_int);
}

pub fn while_loop(){
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
