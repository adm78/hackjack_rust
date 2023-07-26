use std::fmt;

use crate::resources::hand::Hand;
use crate::resources::card::Card;

pub struct InteractivePlayer {
    name: String,
    hand: Hand
}

trait BasicGameplay {
    fn hit_or_stick(&self) -> bool;
    fn take_card(&mut self, card: Card);
}

impl InteractivePlayer {
    pub fn new(name: &str) -> InteractivePlayer {
        return InteractivePlayer{name: name.to_string(), hand: Hand::new(vec![])}
    }
// }


// impl BasicGameplay for InteractivePlayer {

    pub fn take_card(&mut self, card: Card){
        self.hand.add_card(card);
    }

    pub fn hit_or_stick(&self) -> bool {
        let mut line = String::new();
        println!("Hit or stick? (h/s): ");
        std::io::stdin().read_line(&mut line).unwrap();
        println!("Answer: {}", line);
        if line == "h\n" {
            return true
        }
        return false
    }

    pub fn is_bust(&self) -> bool {
        return self.hand.is_bust()
    }
}

impl fmt::Display for InteractivePlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", &self.name, &self.hand)
    }
}