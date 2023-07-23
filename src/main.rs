use crate::resources::card::Card;
use crate::resources::hand::Hand;
use crate::resources::player::Player;

pub mod resources;

fn main() {
    println!("Hello, world!");
    let card1 = Card::new("10", 'D');
    let card2 = Card::new("3", 'S');

    println!("{}", card1.to_string());
    let hand = Hand::new(vec![card1, card2]);
    println!("{}", hand.to_string());
    
    let mut p1 = Player::new("Player 1");
    let card3 = Card::new("9", 'C');
    let card4 = Card::new("A", 'H');
    let card5 = Card::new("K", 'D');
    p1.take_card(card3);
    println!("{}", p1.to_string());
    p1.take_card(card4);
    println!("{}", p1.to_string());
    p1.take_card(card5);
    println!("{}", p1.to_string());
}
