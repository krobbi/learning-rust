use std::io;

// Conversion constants.
const FAHRENHEIT_DIFFERENCE: f64 = 32.0;
const FAHRENHEIT_FACTOR: f64 = 1.8;
const KELVIN_DIFFERENCE: f64 = 273.15;

// A unit of temperature. May be passed by value and tested for equality.
#[derive(Clone, Copy, PartialEq)]
enum Unit {
    CELSIUS,
    FAHRENHEIT,
    KELVIN,
}

// Read a temperature and unit from user input.
fn read_temperature() -> (f64, Unit) {
    loop {
        println!("Enter a temperature (e.g. 20c, 70f, 300k):");
        
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");
        temperature = temperature.trim().to_string();
        
        let unit = match temperature.pop() {
            Some('C') | Some('c') => Unit::CELSIUS,
            Some('F') | Some('f') => Unit::FAHRENHEIT,
            Some('K') | Some('k') => Unit::KELVIN,
            _ => {
                println!("Expected a 'c', 'f' or 'k' unit.");
                continue;
            }
        };
        
        let temperature: f64 = match temperature.trim().parse() {
            Ok(temperature) => temperature,
            Err(_) => {
                println!("Expected a numerical temperature.");
                continue;
            }
        };
        
        if !temperature.is_finite() {
            println!("Temperature cannot be infinite or NaN.");
            continue;
        }
        
        return (temperature, unit);
    }
}

// Convert a temperature from a source unit to a target unit.
fn convert_temperature(temperature: f64, source: Unit, target: Unit) -> f64 {
    // Skip conversion if source and target units match.
    if source == target {
        return temperature;
    }
    
    // Normalize temperature to Celsius.
    let temperature = match source {
        Unit::CELSIUS => temperature,
        Unit::FAHRENHEIT => (temperature - FAHRENHEIT_DIFFERENCE) / FAHRENHEIT_FACTOR,
        Unit::KELVIN => temperature - KELVIN_DIFFERENCE,
    };
    
    // Convert from Celsius to target unit.
    match target {
        Unit::CELSIUS => temperature,
        Unit::FAHRENHEIT => temperature * FAHRENHEIT_FACTOR + FAHRENHEIT_DIFFERENCE,
        Unit::KELVIN => temperature + KELVIN_DIFFERENCE,
    }
}

fn main() {
    let (temperature, unit) = read_temperature();
    println!("Celsius: {}", convert_temperature(temperature, unit, Unit::CELSIUS));
    println!("Fahrenheit: {}", convert_temperature(temperature, unit, Unit::FAHRENHEIT));
    println!("Kelvin: {}", convert_temperature(temperature, unit, Unit::KELVIN));
}
