use std::fmt;

use crate::resources::card::Card;
use crate::resources::hand::Hand;
use crate::resources::gameplay::BasicGameplay;
use crate::resources::utils::is_bust;

pub struct Dealer {
    pub name: String,
    pub hand: Hand
}

impl Dealer {
    pub fn new() -> Dealer {
        return Dealer{ name: "Dealer".to_string(), hand: Hand::new(vec![]) }
    }

    pub fn to_string_full_hand(&self) -> String {
        return format!("{}: {}", &self.name, &self.hand)
    }
}

impl BasicGameplay for Dealer {
    fn take_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    fn hit_or_stick(&self) -> bool {
        if self.hand.value() < 17 {
            return true;
        }
        return false;
    }

    fn is_bust(&self) -> bool {
        return is_bust(&self.hand)
    }
}

impl fmt::Display for Dealer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = self.hand.first_card();
        match result {
            Some(card) => write!(f, "{}: {}", &self.name, card),
            _ => write!(f, "{}: ", &self.name)
        }
        
    }
}