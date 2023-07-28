use std::fmt;

use crate::resources::hand::Hand;
use crate::resources::card::Card;
use crate::resources::gameplay::BasicGameplay;
use crate::resources::utils::is_bust;

pub struct InteractivePlayer {
    pub name: String,
    pub hand: Hand
}

impl InteractivePlayer {
    pub fn new(name: &str) -> InteractivePlayer {
        return InteractivePlayer{name: name.to_string(), hand: Hand::new(vec![])}
    }
}


impl BasicGameplay for InteractivePlayer {

    fn take_card(&mut self, card: Card){
        self.hand.add_card(card);
    }

    fn hit_or_stick(&self) -> bool {
        let mut line = String::new();
        println!("Hit or stick? (h/s): ");
        std::io::stdin().read_line(&mut line).unwrap();
        if line == "h\n" {
            return true
        }
        return false
    }

    fn is_bust(&self) -> bool {
        return is_bust(&self.hand)
    }
}

impl fmt::Display for InteractivePlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", &self.name, &self.hand)
    }
}