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

    pub fn value(&self) -> i32 {
        let mut total = 0;
        let mut ace_count = 0;
        for card in self.cards.iter() {
            if card.is_ace() {
                ace_count += 1;  // we sort these at the end
            } else {
                total += card.value();   
            }
        }

        // handle the aces
        if ace_count > 0 {
            if total <= 10 {
                total += 11 + (ace_count - 1);  // only one ace can have value of 11
            } else {
                total += ace_count;
            }
        }
        total
    }

    pub fn is_bust(&self) -> bool {
        self.value() > 21
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string: String = "".to_string();
        for card in &self.cards {
            string += &(card.to_string() + " ");
        }
        write!(f, "{}({})", string, self.value())
    }
}


// impl Copy for Hand {}

// impl Clone for Hand { fn clone(&self) -> Self { *self } }