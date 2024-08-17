use std::io;

enum PlayResult {
    Win,
    Lose,
    Continue,
}
struct GameState {
    word: String,
    letters: Vec<char>,
    attempts: u32,
}

impl GameState {
    pub fn new(word: String, attempts: u32) -> Self {
        GameState {
            word,
            letters: Vec::new(),
            attempts,
        }
    }
    pub fn palabra_actual(&self) -> String {
        let mut palabra = vec!['_'; self.word.len()];

        for (index, letter) in self.word.chars().enumerate() {
            if self.letters.contains(&letter) {
                palabra[index] = letter;
            }
        }

        palabra.iter().collect()
    }

    pub fn letras_adivinadas(&self) -> Vec<char> {
        self.letters
            .clone()
            .into_iter()
            .filter(|letra| self.word.contains(*letra))
            .collect()
    }

    pub fn play(&mut self, letra: char) -> PlayResult {
        self.letters.push(letra);

        if !self.word.contains(letra) {
            self.attempts -= 1;
        }

        if self.palabra_actual() == self.word {
            PlayResult::Win
        } else if self.attempts == 0 {
            PlayResult::Lose
        } else {
            PlayResult::Continue
        }
    }
}

// La palabra hasta el momento es: _ _ _ _ _ _
// Adivinaste las siguientes letras:
// Te quedan 5 intentos.
fn mostrar_estado(state: &GameState) {
    println!("La palabra hasta el momento es: {}", state.palabra_actual());
    println!(
        "Adivinaste las siguientes letras:{}",
        state
            .letras_adivinadas()
            .iter()
            .fold(String::new(), |acc, letra| format!("{} {}", acc, letra))
    );
    println!("Te quedan {} intentos.", state.attempts);
}

fn main() {
    println!("Bienvenido al ahorcado de FIUBA!\n");

    let mut state = GameState::new(String::from("papa"), 5);

    loop {
        let mut letra = ' ';
        let mut correcta = false;

        mostrar_estado(&state);
        while correcta == false {
            let mut buff = String::new();
            println!("\nIngresa una letra: ");
            io::stdin()
                .read_line(&mut buff)
                .expect("Error al leer la línea");

            if buff.trim().len() > 1 {
                continue;
            }

            letra = buff
                .trim()
                .chars()
                .next()
                .expect("No se ingresó ninguna letra");

            if letra.is_alphabetic() {
                correcta = true;
            }
        }

        match state.play(letra) {
            PlayResult::Win => {
                mostrar_estado(&state);
                println!("\nGanaste!");
                break;
            }
            PlayResult::Lose => {
                println!("\nTe quedaste sin intentos.");
                break;
            }
            PlayResult::Continue => continue,
        }
    }
}
