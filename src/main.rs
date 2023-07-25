use crate::resources::deck::Deck;
use crate::resources::player::Player;

pub mod resources;

fn main() {
    println!("Welcome to Hackjack 3.0");
    println!("-----------------------");

    let mut p1 = Player::new("Player 1");
    let mut p2 = Player::new("Player 2");

    let mut deck = Deck::new();
    println!("{}",deck.to_string());
    deck.shuffle();
    println!("{}",deck.to_string());

    p1.take_card(deck.draw_card());
    p2.take_card(deck.draw_card());
    p1.take_card(deck.draw_card());
    p2.take_card(deck.draw_card());

    println!("{}", p1.to_string());
    println!("{}", p2.to_string());
    
    // let card1 = Card::new("10", 'D');
    // let card2 = Card::new("3", 'S');

    // println!("{}", card1.to_string());
    // let hand = Hand::new(vec![card1, card2]);
    // println!("{}", hand.to_string());
    
    // let mut p1 = Player::new("Player 1");
    // let card3 = Card::new("9", 'C');
    // let card4 = Card::new("A", '♣');
    // let card5 = Card::new("K", '♦');
    // let card6 = Card::new("A", '♥');
    // let card7 = Card::new("A", '♠');
    // p1.take_card(card3);
    // println!("{}", p1.to_string());
    // p1.take_card(card4);
    // println!("{}", p1.to_string());
    // p1.take_card(card5);
    // println!("{}", p1.to_string());
    // p1.take_card(card6);
    // println!("{}", p1.to_string());
    // p1.take_card(card7);
    // println!("{}", p1.to_string());

    // todo: we need some unit test for the value stuff
    // for _ in 1..53 {
    //     deck.draw_cards();
    // }


}
