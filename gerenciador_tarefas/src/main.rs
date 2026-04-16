use std::collections::HashMap;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Clone)]
struct Register {
    title: String,
    status: String
}

#[derive(Subcommand)]
enum Commands {
    Add {
        id: u8,
        title: String,
        status: String,
    },
    List { id: u8 }
}

impl Commands {
    fn exec(&self, todo_list: &mut HashMap<u8, Register>) {
        match self {
            Commands::Add { id, title, status } => {
                let register = Register {
                    title: title.clone(),
                    status: status.clone(),
                };
                
                todo_list.insert(id.clone(), register.clone());
                println!("Tarefa {} salva com sucesso | {}", id, title);
            },
            Commands::List { id } => {
                if let Some(reg) = todo_list.get(id) {
                    println!("ID: {} | Title: {} | Status: {}", id, reg.title, reg.status);
                } else {
                    println!("Chave não encontrada");
                }
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let mut todo_list: HashMap<u8, Register> = HashMap::new();

    todo_list.insert(1, Register { title: String::from("Estudar Rust"), status: String::from("Pendente") });

    cli.command.exec(&mut todo_list);
}
