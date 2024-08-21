use std::{
    collections::HashMap,
    io::{self, Write},
};

struct Document {
    id: u8,
    content: String,
}

fn load_documents() -> Vec<Document> {
    vec![
        Document {
            id: 0,
            content: String::from("El gato negro saltó sobre el tejado."),
        },
        Document {
            id: 1,
            content: String::from("El perro ladró fuerte en la noche."),
        },
        Document {
            id: 2,
            content: String::from("El gato y el perro se miraron fijamente."),
        },
    ]
}

fn remove_stop_words(tokens: Vec<String>) -> Vec<String> {
    let stop_words = ["el", "la", "los", "las", "de", "y", "en", "se"];

    tokens
        .into_iter()
        .filter(|token| !stop_words.contains(&token.as_str()))
        .collect()
}

fn tokenize(text: &str) -> Vec<String> {
    let tokens = text
        .split_whitespace() // Separa las palabras por espacio.
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric())) // Elimina caracteres no alfanumericos al principio o final de cada palabra.
        .map(|s| s.to_lowercase())
        .collect();

    remove_stop_words(tokens)
}

fn build_inverted_index(documents: &Vec<Document>) -> HashMap<String, Vec<u8>> {
    let mut index = HashMap::new();

    for document in documents {
        let tokens = tokenize(&document.content);

        for token in tokens {
            index.entry(token).or_insert(Vec::new()).push(document.id);
        }
    }

    index
}

fn compute_score(
    query: &str,
    index: &HashMap<String, Vec<u8>>,
    num_documents: u8,
) -> HashMap<u8, f64> {
    let query_tokens = tokenize(query);
    let mut doc_scores = HashMap::new();

    for token in query_tokens {
        if let Some(doc_ids) = index.get(&token) {
            let idf = (num_documents as f64 / (1 + doc_ids.len()) as f64).ln();

            for &doc_id in doc_ids {
                let tf = doc_ids.iter().filter(|&&id| id == doc_id).count() as f64;
                let tfidf = tf * idf;

                *doc_scores.entry(doc_id).or_insert(0.0) += tfidf;
            }
        }
    }

    doc_scores
}

fn main() {
    let documents = load_documents();
    let index = build_inverted_index(&documents);

    print!("Enter the search query: ");
    io::stdout().flush().unwrap();

    let mut query = String::new();

    io::stdin()
        .read_line(&mut query)
        .expect("Failed to read line");

    let results = compute_score(&query, &index, documents.len().try_into().unwrap_or(0));

    let ranked_results = rank_results(results);

    for (doc_id, score) in ranked_results {
        println!("Documento ID: {}, Puntaje: {}", doc_id, score);
    }
}

fn rank_results(results: HashMap<u8, f64>) -> Vec<(u8, f64)> {
    let mut ranked_results: Vec<(u8, f64)> = results.into_iter().collect();

    ranked_results.sort_by(|a, b| b.1.total_cmp(&a.1));

    ranked_results
}
