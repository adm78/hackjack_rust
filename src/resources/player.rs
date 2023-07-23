use std::fmt;

use crate::resources::hand::Hand;
use crate::resources::card::Card;

pub struct Player {
    name: String,
    hand: Hand
}

impl Player {
    pub fn new(name: &str) -> Player {
        return Player{name: name.to_string(), hand: Hand::new(vec![])}
    }

    pub fn give_card(&mut self, card: Card){
        self.hand.add_card(card);
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", &self.name, &self.hand)
    }
}