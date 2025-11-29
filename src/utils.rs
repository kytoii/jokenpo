// ---------------------------
// Esse é o módulo utils, para
// utilitários importantes.

// Crates ----------------
use std::process::Command;

// Para limpar a tela
pub fn clear() {

    Command::new("clear")
        .status()
        .unwrap();

}
