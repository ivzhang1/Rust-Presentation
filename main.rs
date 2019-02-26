//Basic Hello World function
fn hello_world(){
    println!("hello world");//double quotation marks must be used to denote a String
}

//Learn to use functions
fn return_number_function(x: i32) -> i32 {
    //Parameter types must be declared
    //Return type must be declared
    return x;
    let x = 50; // this code doesn't run because the function ends as soon as something is returned
}

//Learn to use variables!
fn variables(){
    let num = 45; //let is used to define a variable
    println!("The variable num is defined as {}.", num);
    println!("Num is immutable so its value can't be changed.");
    //num = 100; //all variables are naturally immutable, uncomment this line to crash the function
    let mut another_num = 100; //mut keyword is used to make a variable immutable
    println!("The variable another_num is currently defined as {}.", another_num);
    another_num = 30;//Note that this doesn't crash the program
    println!("The variable another_num is now defined as {}. Note that it was able to be changed because another_num was defined to be mutable.", another_num);
}

//Learn about the different types of primitives
fn primitives(){
    //All variables must be assigned to a specific primitive or data type in Rust. Rust will do this automatically. See below for the list of all possible primitives
    //bool: The boolean type.
    //char: A character type.
    //i8: The 8-bit signed integer type.
    //i16: The 16-bit signed integer type.
    //i32: The 32-bit signed integer type.
    //i64: The 64-bit signed integer type.
    ///isize: The pointer-sized signed integer type.
    //u8: The 8-bit unsigned integer type. Doesn't support negative numbers
    //u16: The 16-bit unsigned integer type. Doesn't support negative numbers
    //u32: The 32-bit unsigned integer type. Doesn't support negative numbers
    //u64: The 64-bit unsigned integer type. Doesn't support negative numbers
    //usize: The pointer-sized unsigned integer type. Doesn't support negative numbers
    //f32: The 32-bit floating point type.
    //f64: The 64-bit floating point type.
    //array: A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
    //slice: A dynamically-sized view into a contiguous sequence, [T].
    //str: String slices.
    //tuple: A finite heterogeneous sequence, (T, U, ..).
    //Instead of allowing Rust to assign primitive type to a variable you can define it yourself. See below:
    let x = 30; //will default to a 32-bit integer
    let y = 30.1; //will default to a 32-bit float
    let mut z: u64 = 3; //set to a 64-bit unsigned integer
    //z = -1; //delete the commenting to see this crash the program because z is an unsigned integer
}

//Learn to use conditionals
fn conditionals(num: i32){
    if num == 5 { //parentheses aren't needed around the boolean
        println!("Num is five!");
    }
    else if num == 6 {
        println!("Num is six!");
    }
    else {
        println!("Num is not five or six :(");
    }
}

fn forever_loop(){
    loop {
        println!("Loop forever!");
    }
}

//Learn how to use while loops
fn fact_while_loop(num: i32){
    let mut n = num; // must make changing variable mutable
    let mut answer = 1;
    while n > 1{
        answer = answer * n;
        n = n - 1;
    }
    println!("Answer is {}", answer);
}

//Learn how to use for loops
fn fact_for_loop(num: i32){
    let mut answer = 1;
    for n in 1..num + 1{ //ranges from 1 to num - 1
        answer = answer * n;
    }
    println!("Answer is {}", answer);
}

//Learn to use structs as a way to create custom data structures; similar to classes in Java
struct Bank_account {
    name: String,//note each variables type
    deposit_amount: i32
}

fn structures(){
    let account = Bank_account {name: "Jerry".to_string(), deposit_amount: 1000 };
    println!("{} has {} in total deposits.",account.name, account.deposit_amount);
    //account.name = "Ivan".to_string(); //this will crash the program because account is immutable
    let mut acc = Bank_account {name: "Ivan".to_string(), deposit_amount: 0 };
    println!("{} has {} in total deposits.",acc.name, acc.deposit_amount);
    acc.name = "Jerry".to_string();//changes name instance variable
    println!("{} has {} in total deposits.",acc.name, acc.deposit_amount);

}

fn main(){
    //uncomment these to begin learnination
    //hello_world();
    //println!("Number is: {}", return_number_function(50));
    //variables();
    //primitives();
    //conditionals(5);
    //forever_loop();
    //fact_while_loop(5);
    //fact_for_loop(5);
    structures()

}
