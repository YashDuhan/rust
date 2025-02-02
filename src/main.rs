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

fn mutability(){
    let num = 5;
    // num = 32; -> will return error
    // to change it 
    // declare with mut
    println!("Num : {}", num);
    let mut num1 = 5;
    println!("Before changing:  {} ", num1);
    num1 = 20;
    println!("After changing: {}", num1);
}

fn arr_slice_ops(arr: [u8; 6], slice: &[u8]){
    println!("Arr : {:?}", arr);
    println!("Slice: {:?}", slice);
    println!("Slice length: {:?}", slice.len());
    println!("Index 0: {}, Index 1 : {}", slice[0], slice[1]);
}

// Arrays and slices
fn fn1(){
    let arr = [0,1,2,3,4,5];
    // 1st val is inclusive and last is exclusive 
    let slice = &arr[1 .. 5]; //1,2,3,4; we don't know length hence using & to reference it

    arr_slice_ops(arr, slice);
}

fn string(){
    // let str: &str = "Hello world";
    let mut string: String = String::from("Hello world"); //string object: It's like a vector; size can increase

    // get everything from 0 to index 6; index 6 excluded
    let slice = &string[.. 6];
    println!("Slice length : {} , Slice : {:?}", slice.len(), slice);

    string.push('A');
    string.push_str("Yash");

    string = string.replace("Hello", "Bye");

    println!("{}", string);
}

fn if_else(){
    let n = 3;
    if n > 0{
        println!(" Greater than 0");
    }else if n < 0 {
        println!(" Less than 0");
    }else {
        println!(" Is 0");
    }
}

fn for_loop(){
    for i in 0..6{
        println!("{}",i);
    }
}

fn while_loop(){
    let mut i = 0;
    while i < 5 {
        println!("{}", i);
        i+=1;
    }
}

fn swtich_statement(){
    let i = 4;
    match i {
        0 => println!("0"),
        1 | 2 => println!("Either 1 or 2"),
        3..=4 => println!("Value between 3 and 4; 4 is included"),
        _ => println!("Default")
    }
}

// Structs
struct Bird{
    name: String,
    age : u128
}
// implement methods in struct
impl Bird {
    fn print_name(&self){
        println!("Name : {}", self.name);
    }
    fn print_age(&self){
        println!("Age: {}", self.age)
    }
}

// Interfaces in Rust (Traits)
trait Animal{
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool{
        return true;
    }
}

impl Animal for Bird{
    fn can_fly(&self)-> bool{
        return true;
    }
}

// Enum
#[derive(Debug)]
enum MyEnum{
    A,
    B(i32),
    C{x:i32, y:i32}
}

// vectors
fn vectors(){
    let mut vec: Vec<i64> = vec![1,2,3,4,5];
    vec.len();
    vec[0];
    vec.push(6);
    vec.remove(0);
    println!("{:?}", vec);

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

    mutability();
    fn1();
    string();
    if_else();
    for_loop();
    while_loop();
    swtich_statement();

    // using a struct
    let name_bird = String::from("Yash Bird");
    let bird = Bird{name: name_bird, age: 20};
    bird.print_name();
    bird.print_age();
    println!("Can Fly: {}, is Animal : {} ", bird.can_fly(), bird.is_animal());

    let a : MyEnum = MyEnum::A;
    let b : MyEnum = MyEnum::B(5);
    let c : MyEnum = MyEnum::C { x: 10, y: 20 };
    println!("A: {:?}", a);
    println!("B: {:?}", b);
    println!("C: {:?}", c);

    if let MyEnum::B(val) = b{
        println!("Value for B : {}", val);
    }

    if let MyEnum::C{x,y} = c {
        println!(" C(X) : {} C(Y) : {}", x, y);
    } 

    vectors();

}
