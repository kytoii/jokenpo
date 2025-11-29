// -------------------
// Developed by Kytoii

// Meu código é minha vida -
// My code is my life -

mod logic;  // Módulo para lógica
mod io;     // Módulo de entrada
mod output; // Módulo de saída
mod utils;  // Utilitários

fn main() {

    // Essa é a função principal.
    // O sistema nervoso do nosso
    // programa.

    utils::clear();
    output::init(); // Diz ao output que o 
                    // programa foi iniciado.

    // Pega a entrada do usuário.
    let input: i32 = io::input();

}
