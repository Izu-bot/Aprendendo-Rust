pub fn celsius_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}

pub fn fahrenheit_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

pub fn celsius_kelvin(celsius: f32) -> f32 {
    celsius + 273.15
}

pub fn fahrenheit_kelvin(fahrenheit: f32) -> f32 {
    (((fahrenheit - 32.0) * 5.0) / 9.0) + 273.15
}

pub fn kelvin_celcius(kelvin: f32) -> f32 {
    kelvin - 273.15
}

pub fn kelvin_fahrenheit(kelvin: f32) -> f32 {
    ((kelvin - 273.15) * 1.8) + 32.0
}
