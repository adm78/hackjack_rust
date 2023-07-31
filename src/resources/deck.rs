use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;
use log::debug;

use crate::resources::card::Card;
use crate::resources::constants::{SUITS, FACE_CARDS, ACE};

pub struct Deck {
    cards: Vec<Card>,
    count: i16
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = vec![];
        for suit in SUITS.iter() {
            for i in 2..11 {
                cards.push(Card::new(&i.to_string(), *suit));
            } 
            for i in FACE_CARDS.iter() {
                cards.push(Card::new(i, *suit));
            }
            cards.push(Card::new(&ACE, *suit));
            
        }
        return Deck{cards: cards, count: 0}  
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn draw_card(&mut self) -> Card {
        let result = self.cards.pop();
        match result {
            Some(card) => {
                self.count += card.count_weighting();
                debug!("Current count = {}, norm count = {}.", self.count, self.normalised_count());
                card
            },
            None => {
                panic!("Whoops. There are no cards left in the deck :/")
            }
        }
    }

    pub fn normalised_count(&self) -> f32 {
        return f32::from(self.count)/(f32::from(self.cards.len() as i16)/52.0)
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