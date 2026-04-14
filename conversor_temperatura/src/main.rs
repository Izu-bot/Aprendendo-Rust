use clap::{Args, Parser, Subcommand};
mod conversor;

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Celsius(Temperatura),
    Fahrenheit(Temperatura),
    Kelvin(Temperatura),
}

#[derive(Args)]
struct Temperatura {
   temperatura: f32
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Celsius(args) => {
            println!("Convertendo C:");

            let cf = conversor::celsius_fahrenheit(args.temperatura);
            let ck = conversor::celsius_kelvin(args.temperatura);

            println!("Celsius -> Fahrenheit: {}\nCelsius -> Kelvin: {}", cf, ck);
        }
        Commands::Fahrenheit(args) => {
            println!("Convertendo F:");

            let fc = conversor::fahrenheit_celsius(args.temperatura);
            let fk = conversor::fahrenheit_kelvin(args.temperatura);

            println!("Fahrenheit -> Celsius: {}\nFahrenheit -> Kelvin: {}", fc, fk);
        }
        Commands::Kelvin(args) => {
             println!("Convertendo K:");

            let kc = conversor::kelvin_celcius(args.temperatura);
            let kf = conversor::kelvin_fahrenheit(args.temperatura);

            println!("Kelvin -> Celsius: {}\nKelvin -> Fahrenheit: {}", kc, kf);         
        }
    }
}
