use std::fmt;

use crate::resources::card::Card;

pub struct Hand {
    cards: Vec<Card>
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        return Hand{cards: cards}
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string: String = "".to_string();
        for card in &self.cards {
            string += &(card.to_string() + " ");
        }
        write!(f, "{}", string)
    }
}


// impl Copy for Hand {}

// impl Clone for Hand { fn clone(&self) -> Self { *self } }