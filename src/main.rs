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

fn main() {
    println!("Hello, world!");
    let a = 10; 
    let b = 15;
    // {} serves as the place holders
    println!("A is {} and B is {}", a , b);
    
    types();
    arrays();
}
