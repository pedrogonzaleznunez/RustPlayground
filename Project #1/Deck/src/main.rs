#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

// inherent implementation
impl Deck {
    fn new() -> Self {

        // array of 'suits' and values
        let suits = ["Hearts","Spades","Diamonds"];
        let values = ["Ace","Two","Three"];

        let mut cards = vec![]; // cards: Vec::new() // generastes empty vector as well

        for suit in suits {
            for value in values {
                let card = format!("{} of {}",value, suit);
                cards.push(card);
            }
        }

        let deck:Deck = Deck { cards: cards}; // = Deck { cards};
        return deck;
    }
}

fn main() {

    let deck = Deck::new();
    println!("Here's ur deck {:#?}", deck);
    // println!("Here's ur deck {deck}");

}
