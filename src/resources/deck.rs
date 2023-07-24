use std::fmt;

use crate::resources::card::Card;
use crate::resources::constants::{SUITS, FACE_CARDS, ACE};

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = vec![];
        for suit in SUITS.iter() {
            for i in 2..10 {
                cards.push(Card::new(&i.to_string(), *suit));
            } 
            for i in FACE_CARDS.iter() {
                cards.push(Card::new(i, *suit));
            }
            cards.push(Card::new(&ACE, *suit));
            
        }
        return Deck{cards: cards}  
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string: String = "".to_string();
        for card in &self.cards {
            string += &(card.to_string() + " ");
        }
        write!(f, "{}", string)
    }
}