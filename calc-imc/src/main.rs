use std::io;

fn main() {
    println!("Calculador de IMC");
 
    println!("Digite seu peso:");        
    
    let input = ler_dados();

    let peso: f32 = input.trim().parse().expect("Falha ao ler o peso.");

    println!("Digite sua altura:");

    let input = ler_dados();

    let altura: f32 = input.trim().parse().expect("Falha ao ler a altura.");
    
    println!("Seu IMC é: {}", calc_imc(peso, altura));

}

fn calc_imc(peso: f32, altura: f32) -> f32 {
    peso / (altura * altura)
}

fn ler_dados() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    input
}
