use rand::seq::SliceRandom;

fn main() {
    let quotes = get_quotes();
    let mut rng = rand::thread_rng();
    if let Some(quote) = quotes.choose(&mut rng) {
        println!("💡 Quote of the Day:\n{}", quote);
    }
}

fn get_quotes() -> Vec<&'static str> {
    vec![
        "Be yourself; everyone else is already taken.",
        "The only limit to our realization of tomorrow is our doubts of today.",
        "Do one thing every day that scares you.",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quotes_not_empty() {
        let quotes = get_quotes();
        assert!(!quotes.is_empty(), "Quotes list should not be empty");
    }

    #[test]
    fn test_quotes_content() {
        let quotes = get_quotes();
        assert!(quotes.contains(&"Be yourself; everyone else is already taken."));
        assert!(quotes.contains(&"The only limit to our realization of tomorrow is our doubts of today."));
        assert!(quotes.contains(&"Do one thing every day that scares you."));
    }
}