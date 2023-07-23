use std::fmt;

pub struct Card {
    value: String,
    suit: char
}

impl Card {
    pub fn new(value: &str, suit: char) -> Card {
        return Card{value: value.to_string(), suit: suit}
    }

    // pub fn to_string(&self) {
    //     format!("{value}{suit}", value=self.value, suit=self.suit);
    // }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}