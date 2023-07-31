use crate::resources::dealer::Dealer;
use crate::resources::deck::Deck;
use crate::resources::player::InteractivePlayer;
use crate::resources::gameplay::BasicGameplay;
use env_logger;

pub mod resources;

// env_logger::init();

fn main() {
    env_logger::init();
    println!("-----------------------");
    println!("Welcome to Hackjack 3.0");
    println!("-----------------------");
    // https://bicyclecards.com/how-to-play/blackjack/

    let mut p1 = InteractivePlayer::new("Player 1");
    let mut dealer = Dealer::new();

    let mut deck = Deck::new();
    println!("{}",deck.to_string());
    deck.shuffle();
    println!("{}",deck.to_string());

    p1.take_card(deck.draw_card());
    dealer.take_card(deck.draw_card());
    p1.take_card(deck.draw_card());
    dealer.take_card(deck.draw_card());

    println!("{}", p1.to_string());
    println!("{}", dealer.to_string());


    // player hit or stick
    while !p1.is_bust() {
        if p1.hit_or_stick() {
            p1.take_card(deck.draw_card());
            println!("{}", p1.to_string());
        } else {
            break;
        }
    }
    if p1.is_bust() {
        println!("{} is bust!", p1.name);
    } else {
        println!("{}", p1.to_string());
    }

    while !dealer.is_bust() {
        if dealer.hit_or_stick() {
            dealer.take_card(deck.draw_card());
            println!("{}", dealer.to_string_full_hand());
        } else {
            break;
        }
    }
    if dealer.is_bust() {
        println!("{} is bust!", dealer.name);
    } else {
        println!("{}", dealer.to_string_full_hand());
    }

    if p1.is_bust() {
        println!("{} loses!", p1.name);
    } else {
        if dealer.is_bust() {
            if p1.hand.is_blackjack() {
                println!("{} has blackjack!", p1.name);
            }
            println!("{} wins!", p1.name);
        } else {
            if p1.hand.is_blackjack() && dealer.hand.is_blackjack() {
                println!("{} and {} both have blackjack. It's a draw.", p1.name, dealer.name);
            } else if p1.hand.value() > dealer.hand.value() {
                println!("{} wins!", p1.name);
            } else {
                println!("{} loses!", p1.name);
            }
        }
    }
}
