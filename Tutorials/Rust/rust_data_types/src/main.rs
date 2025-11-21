//Required to use Hashmaps
use std::collections::HashMap;

fn main() {
    //Arrays
    // Has a fixed size, index starts at 0
    let fruits_array = ["apple", "banana", "orange"];
    println!("Last Fruit: {}", fruits_array[2]);
    // Arrays must have the "mut" keyword if a value inside them is to be changed at runtime
    // fruits_array[0] = "strawberry"; wouldn't work
    let mut numbers = [1, 2, 3, 4, 5];
    numbers[0] = 10;
    println!("The new first number is: {}", numbers[0]);
    println!("This array has {} elements", numbers.len());

    //Loop through array
    for fruit in fruits {
        println!("I like {}.", fruit);
    }

    //Print whole array
    println!("{:?}", numbers);
    

    //Vectors
    // the vec! macro denotes what is a vector
    let mut fruits_vector = vec!["apple", "banana"];
    //Vectors are the same as arrays except they are dynamic
    // so they can increase or decrease in size
    fruits_vector.push("cherry"); //Add to the end
    fruits_vector.pop(); //Remove from the end
    fruits_vector.insert(0, "apple"); //Add at index
    fruits_vector.remove(0); //Remove from index
    println!("There are {} fruits", fruits_vector.len());
    println!("Last Fruit: {}", fruits_vector[2]);

    //Loop through vector
    for fruit in &fruits_vector { //Must use & here to borrow the data rather than stealing it
        println!("I like {}", fruit);
    }

    //Tuples
    // Can store multiple different data types
    let person = ("John", 30, true);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);

    //Hashmaps
    let mut capitalCities = HashMap::new();
    capitalCities.insert("France", "Paris");
    capitalCities.insert("Japan", "Tokyo");

    println!("Capital of Japan: {}", capitalCities["Japan"]);
}
