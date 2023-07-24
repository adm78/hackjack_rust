use once_cell::sync::Lazy;

pub static FACE_CARDS: Lazy<Vec<String>> = Lazy::new(||vec!["J".to_string(), "Q".to_string(), "K".to_string()]);
pub static ACE: Lazy<String> = Lazy::new(||"A".to_string());
pub static SUITS: [char; 4] = ['♠', '♥', '♦', '♣'];