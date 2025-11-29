// -----------------------------
// Esse módulo é responsavel por
// printar informações na tela.

// Crates ------------
use colored::Colorize;
use std::io::{stdout, Write};

// Início
pub fn init() {

    // Título
    println!(
        "{} {} {}",
        "[".red(),
        "Jokenpô in Rust".yellow(),
        "]".red()
    );

    println!(); // Linha em branco
    
    // Start 
    println!(
        "{}{}{} {} {}",
        "[".red(),
        "1".yellow(),
        "]".red(),
        "-".red(),
        "Start".yellow()
    );

    // Exit
    println!(
        "{}{}{} {} {}",
        "[".red(),
        "2".yellow(),
        "]".red(),
        "-".red(),
        "Exit".yellow()
    );

    println!(); // Linha em branco

    // Entrada
    print!(
        "{}{}{} {} {} ",
        "[".red(),
        "You".yellow(),
        "]".red(),
        "-".red(),
        ">".yellow(),
    );

    // Isso garante que o print
    // saía corretamente -----
    stdout().flush().unwrap();

}
