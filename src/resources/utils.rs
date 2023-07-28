use crate::resources::hand::Hand;

pub fn is_bust(hand: &Hand) -> bool {
    return hand.value() > 21;
}