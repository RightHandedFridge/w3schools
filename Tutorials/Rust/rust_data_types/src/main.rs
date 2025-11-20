//Required to use Hashmaps
use std::collections::HashMap;

fn main() {
    //Arrays
    // Has a fixed size, index starts at 0
    let fruits_array = ["apple", "banana", "orange"];
    println!("Last Fruit: {}", fruits_array[2]);

    //Vectors
    let mut fruits_vector = vec!["apple", "banana"];
    fruits_vector.push("cherry");
    println!("Last Fruit: {}", fruits_vector[2]);

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
