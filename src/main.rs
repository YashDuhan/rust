fn types(){
    // integers
    // unsigned integers 
    // u8, u16, u32, u64, u128
    // let variable name : type = value
    let unsigned: u8 = 10;

    // signed intgers
    let signed:i8 = -10;

    // float for decimal
    let float:f32 = 1.4;

    print!("Unsigned is {} \nSigned is {} \n float is {}", unsigned, signed, float);

    // chars
    let letter = "c";
    let emojis = "\u{1F600}"; // smiling face emoji

    print!("\nLetter : {} \nEmoji : {}", letter, emojis);

    // boolean
    let is_true:bool = true;

    print!("\nIsTrue: {}", is_true);
}

fn arrays(){
    // u8 is the type and 3 is the size
    let arr:[u8; 3] = [1,2,3];

    let other_arr: [u8; 5] = [100; 5]; //fill 100 5 times

    println!("\nIndex at arr[0]: {} \nLength of 2nd arr: {}", arr[0], other_arr.len());

    // print arr
    print!("\nArr: {:?}", arr);
    print!("\nOther Arr: {:?}", other_arr);
}

fn tuples(){
    // can hold items of diff types
    // int, bool, float
    let tuple: (u8, bool , f32) = (5, true, 2.3);
    let tuple2: (u8, u8) = (3,5);

    // print
    println!("\nFirst: {} , Second: {} , Third: {} ", tuple.0 , tuple.1, tuple.2);
    println!("Tuple 2: {:?}", tuple2);

    // destructuring - extract elements from tuple into vars
    let (a,b,c) = tuple;
    println!("A : {} , B : {} , C : {}", a, b, c);
}

// functions
// By default all the functions are private

// pub : public
// is_even : fn name
// (num: u8) : Args/Params
// -> bool : return type
pub fn is_even(num:u8) -> bool{
    let remainder: u8 = num % 2;
    // both of these return statements will work
    // remainder == 0 // return statement(no semicolons)
    return remainder == 0;
}


fn main() {
    println!("Hello, world!");
    let a = 10; 
    let b = 15;
    // {} serves as the place holders
    println!("A is {} and B is {}", a , b);
    
    types();
    arrays();
    tuples();

    // return
    println!(" Is Even : {}", is_even(12));
}
