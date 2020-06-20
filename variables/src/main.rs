fn main() {
    //Immutability (Won't Compile)
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x); 
    
    //Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    //Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    //This won't compile
    // let mut spaces = "   ";
    // spaces = spaces.len();

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    //Boolean
    let t = true;

    let f: bool = false; 

    //Char Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    //Compound Type

    //Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; //Destructing
    println!("The value of y is: {}", y);

    //Pattern matching
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //Array Type, Collection for similar type, fixed in length
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // a = [3, 3, 3, 3, 3]

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    // Below code will panic due to index out of bounds
    // let index = 10;
    // let element = a[index];

    // println!("The value of element is: {}", element);
    
} 

