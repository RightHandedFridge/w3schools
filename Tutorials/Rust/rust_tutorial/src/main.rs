fn main() {
    //Print statements per line
    println!("Hello, world!");
    println!("I am learning Rust.");
    println!("It is awesome!");

    //Print statement without \n
    print!("Hello, World!");
    print!("I will write this on the same line!");

    //Initialising variables
    let name = "John";
    println!("My first name is: {}", name);

    //Placeholders are replaced in order (i.e name, age not age, name)
    let age = 30;
    println!("{} is {} years old.", name, age);

    //Variables can't be changed by default
    // age = 20; <-- Error
    // To do this, use keyword "mut"
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);

    //Varibles do not require a type to be specified
    //Rust will choose an appropriate type based on the data

    let _my_num = 5; // integer
    let _my_double = 5.99; // float
    let _my_letter = 'D'; // character
    let _my_bool = true; // bool
    let _my_text = "Hello"; // string

    //Types can be specified also
    let _my_num: i32 = 5; // integer
    let _my_double: f64 = 5.99; // float
    let _my_letter: char = 'D'; // character
    let _my_bool: bool = true; // boolean
    let _my_text: &str = "Hello"; // string

    //Constants must have a type specified
    //It is also good to declare them with capital letters
    const BIRTHYEAR: i32 = 1980;
    const MINUTES_PER_HOUR: i32 = 60;

    //Iteration

    if 7 > 5 {
        println!("7 is greater than 5!");
    } else {
        println!("Mathematics have not been invented.");
    }

    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else {
        println!("Grade: F");
    };

    //You can use If as an expression as well... for whatever reason
    let time = 20;
    let greeting = if time < 18 {
        "Good Day."
    } else {
        //Else must always be included so the variable always has a
        "Good Evening."
    };
    //Both results of the if statement must be the same type as well
    // Can't have if -> "Good Day" else -> 500
    println!("{}", greeting);

    //Switch statements are replaced by match statements
    // Rust will force your match statement to consider all options
    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid Day"), //"default" case
    }

    //Match statements have no fallthrough so this syntax will allow multiple values to result in the same output
    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    //Match statements can also be used in expressions
    let result = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid Day",
    };

    println!("{}", result);

    //Loops
    loop {
        println!("This will repeat forever");
        break; //Without this :^)
    }

    let mut count = 1;

    loop {
        println!("Hello World!");

        if count == 3 {
            break;
        };
        count += 1;
    }

    count = 1;

    let loop_result = loop {
        println!("Hello");

        if count == 3 {
            break count;
        }

        count += 1;
    };

    println!("The loop stopped at: {}", result);

    //While Loop
    let mut while_loop_counter = 1;

    while while_loop_counter <= 10 {
        if while_loop_counter == 6 {
            break; //Can be replaced with continue to skip the value
        }
        println!("Number: {}", while_loop_counter);
        while_loop_counter += 1;
    }

    //For Loop
    // Prints numbers 1-5
    for i in 1..6 {
        println!("i is: {}", i);
    }

    //For Inclusive use ..=
    for j in 1..=6 {
        println!("j is: {}", j);
    }

    //Functions
    say_hello();
    greet("John");
    let sum = add(3, 4);
    println!("Sum is: {}", sum);

    //Variables declared within Code blocks (loops, if statements etc) and Functions are out of scope
    //Variables can be replaced with a variable of the same name when not using the mut keyword
    let x = 1;
    let x = 10;

    println!("x is {}", x); //prints 10
}

fn say_hello() {
    println!("Hello from a function!");
}

fn greet(name: &str) {
    println!("Hello, {}", name);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
