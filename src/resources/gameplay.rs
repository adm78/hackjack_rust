use crate::resources::card::Card;

pub trait BasicGameplay {
    fn hit_or_stick(&self) -> bool;
    fn take_card(&mut self, card: Card);
    fn is_bust(&self) -> bool;
}