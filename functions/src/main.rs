fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);
    
    let x = five();
    let x = plus_one(x);

    another_function(x, y);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // Expression
    // x + 1; Statement which will result in compile time error
}

fn five() -> i32 {
    5
}