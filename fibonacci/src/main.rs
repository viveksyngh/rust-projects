fn main() {
    let n = 5;
    let mut fib_1 = 1;
    let mut fib = 1;
    if n <= 2 {
        fib = 1;
    }
    
    let mut counter = 2;
    while counter < n {
        let temp = fib;
        fib = fib_1 + fib;
        fib_1 = temp;
        counter += 1;
    }
    println!("{} fibonacci number is : {}", n, fib);
}
