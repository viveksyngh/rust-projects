fn main() {
    {
        let s = "Hello World!";
        println!("from inside scope {}", s);
    }

    // println!("from outside scope {}", s);  Will give syntax error

    {
        let mut s = String::from("Hello");
        s.push_str(", world!");
        println!("From inside scope {}", s);
    }

    // println!("from outside scope {}", s);  Will give syntax error

    // Move example
    let s1 = String::from("hello"); // s1 is the owner of this value
    let s2 = s1; // s1 moved into s2 (Ownership changes to s2)

    // println!("s1 = {}, s2 = {}", s1, s2); // s1 is no longer valid and will give error

    //Clone example
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    //Stack Only Data
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    //Ownership and functions

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward

    return_value_and_scope();

    //Reference and Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    //Mutable references
    let mut s = String::from("hello");
    change(&mut s);

    //Mutable references
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;

    // println!("{}, {}", r1, r2); //Compile time error

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;


    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3); // Compile time error


    //reference’s scope starts from where it is introduced and continues 
    //through the last time that reference is used
    
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


// Return Value and It's Scope
fn return_value_and_scope() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
