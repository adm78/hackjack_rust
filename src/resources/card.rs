use std::fmt;




pub struct Card {
    value: String,
    suit: char
}

impl Card {
    pub fn new(value: &str, suit: char) -> Card {
        return Card{value: value.to_string(), suit: suit}
    }

    pub fn value(&self) -> i32 {
        let face_cards: Vec<String> = vec!["K".to_string(), "Q".to_string(), "J".to_string()];
        let ace: String = "A".to_string();
        if face_cards.contains(&self.value) {
            return 10
        } else if self.value == ace {
            return 11
        } else {
            self.value.parse::<i32>().unwrap()
        }
    }

    pub fn is_ace(&self) -> bool {
        return self.value == "A"
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