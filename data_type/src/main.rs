fn main() {

    ////Data Types///////
    //arrays can be define
    let arrays : [i32;5] = [1,2,3,4,5];

    //array result [3,3,3,3,3]
    let arrays2=[3;5];

    //char
    let c = 'z';
    let z = 'ℤ';

    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);


    println!("Hello, world!");
}

//////Functions

fn plus_one(x: i32) -> i32 {
    //return the last line without semnicolon
    x + 1
}
Comp
