use std::io;

fn main() {
    println!("Welcome to the BMI calculator! Let's calculate your BMI together.");

    // Get user input for weight in kilograms
    println!("Please enter your weight in kilograms:");
    let mut weight_input = String::new();
    io::stdin().read_line(&mut weight_input).unwrap();
    let weight = weight_input.trim().parse::<f32>().unwrap();

    // Get user input for height in meters
    println!("Now, please enter your height in meters:");
    let mut height_input = String::new();
    io::stdin().read_line(&mut height_input).unwrap();
    let height = height_input.trim().parse::<f32>().unwrap();

    // Calculate BMI
    let bmi = weight / (height * height);

    // Print BMI and interpretation
    println!("Great, your BMI is {:.1}", bmi);
    if bmi < 18.5 {
        println!("According to your BMI, you may be underweight. It's important to make sure you're getting the nutrients you need to maintain a healthy weight.");
    } else if bmi < 25.0 {
        println!("Based on your BMI, it looks like you're in the healthy weight range. Keep up the good work!");
    } else if bmi < 30.0 {
        println!("Your BMI suggests you may be overweight. It's important to maintain a healthy weight to avoid health problems. Consider speaking with a healthcare professional for advice.");
    } else {
        println!("Based on your BMI, you may be obese. Obesity can increase the risk of health problems, so it's important to take steps to maintain a healthy weight. Consider speaking with a healthcare professional for advice.");
    }
}
