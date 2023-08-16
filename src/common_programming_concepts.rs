pub fn run() {
    // variables
    let mut x = 5;
    println!("x : {}", x);
    x = 6;
    println!("new x : {}", x);

    const MYAGE: u32 = 25;

    // shadowing
    let sha = 5;
    println!("sha {}", sha);
    let sha: &str = "six";
    println!("sha {}", sha);

    // data types

    let a = 5;
    let b = 2.5;
    let c = 0xff;

    let sum = 5 + 10;
    let sub = 10 - 5;
    let mul = 10 * 5;
    let div = 10 / 5;

    let char = 'a';
    let stringb = "hello";

    // compound types

    let tup = ("my age is", 25);
    let age = tup.1;

    let arr = [1, 2, 3];

    let z = arr[2];
    println!("{z}");

    // functions
    my_function(11, 22);

    // comments
    // This is a comment.

    // control flow

    // if-else
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in 1..5{
        println!("{}", number)
    }
}

fn my_function(x: i32, y: i32) {
    println!("another function");
    let sum = x + y;
    println!("sum : {}", sum);
}
