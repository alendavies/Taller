use std::{
    cmp::Reverse,
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut linea = String::new();

    let mut map: HashMap<String, u32> = HashMap::new();

    let mut palabras: Vec<String> = Vec::new();

    while reader.read_line(&mut linea)? > 0 {
        palabras = linea
            .to_lowercase()
            .split_whitespace()
            .map(str::to_string)
            .collect();
    }

    for palabra in palabras {
        if !map.contains_key(&palabra) {
            map.insert(palabra.to_string(), 1);
        } else {
            map.entry(palabra.to_string()).and_modify(|v| *v += 1);
        }
    }

    let mut pares: Vec<(&str, u32)> = Vec::new();

    for (clave, valor) in &map {
        pares.push((clave, *valor));
    }
    pares.sort_by_key(|k| Reverse(k.1));

    for (clave, valor) in pares {
        println!("{} -> {}", clave, valor);
    }

    Ok(())
}
