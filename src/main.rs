//mod systeminfo;
//mod userinput;
//mod datatypes;
mod commons;

fn main() {
    println!("Hello, world!");
    //systeminfo::system_information();
    //systeminfo::rust_information();
    //userinput::while_loop();
    //userinput::user_input();
    //datatypes::arrays();
    let factorial =    commons::factorial(5);
    println!("factorial: {}", factorial)
}
