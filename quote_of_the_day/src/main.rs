use rand::seq::SliceRandom;

fn main() {
    let quotes = vec![
        "Be yourself; everyone else is already taken.",
        "The only limit to our realization of tomorrow is our doubts of today.",
        "Do one thing every day that scares you.",
    ];

    let mut rng = rand::thread_rng();
    if let Some(quote) = quotes.choose(&mut rng) {
        println!("💡 Quote of the Day:\n{}", quote);
    }
}
