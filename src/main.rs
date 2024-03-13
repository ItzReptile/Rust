// Import the standard I/O library for input and output operations
use std::io;

// Define the main function, where the program starts execution
fn main() {
    // Print a welcome message and options for selecting a conversion
    println!("Welcome to the Rust Temperature Converter!");
    println!("What would you like to convert?");
    println!("1. Convert Fahrenheit to Celsius");
    println!("2. Convert Celsius to Fahrenheit");

    // Read user input for conversion choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    // Parse the input into an unsignedd 32-bit integer
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            // Handle invalid input
            println!("Invalid choice, please choose either 1 or 2");
            return;
        }
    };

    // Match the user's choice and call the corresponding function
    match choice {
        1 => calculate_fahrenheit_to_celsius(),  // Call the function to convert Fahrenheit to Celsius
        2 => calculate_celsius_to_fahrenheit(),  // Call the function to convert Celsius to Fahrenheit
        _ => println!("Invalid choice"),  // Handle invalid choice
    }
}

// Function to convert Fahrenheit to Celsius
fn calculate_fahrenheit_to_celsius() {
    // Prompt the user to enter the temperature in Fahrenheit
    println!("Enter the temperature in Fahrenheit:");
    // Read user input for temperature in Fahrenheit
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");
    // Parse the input into a floating-point number
    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            // Handle invalid input
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    // Convert Fahrenheit to Celsius using the conversion formula (°F - 32) * 5/9
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    // Print the converted temperature in Celsius
    println!("Temperature in Celsius: {}", celsius);
}

// Function to convert Celsius to Fahrenheit
fn calculate_celsius_to_fahrenheit() {
    // Prompt the user to enter the temperature in Celsius
    println!("Enter the temperature in Celsius:");
    // Read user input for temperature in Celsius
    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Failed to read line");
    // Parse the input into a floating-point number
    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            // Handle invalid input
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    // Convert Celsius to Fahrenheit using the conversion formula (°C * 9/5) + 32
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    // Print the converted temperature in Fahrenheit
    println!("Temperature in Fahrenheit: {}", fahrenheit);
}
