// Esse é o módulo io, para entrada
// e saída da dados.

// Crates ---------
use std::io::stdin;

pub fn input() -> i32 {

    // Cria a váriavel input e coloca o
    // valor da entrada nela -------------
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();

    // Faz o parse para i32 e limpa espaços -------
    let input: i32 = input.trim().parse().unwrap();

    input // Retorna a váriavel ao programa.

}
