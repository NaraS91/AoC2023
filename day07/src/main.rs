use std::{cmp::Ordering, fs};

// a bit overengineered, but was fun (:
const IS_PART2: bool = true;

#[derive(Clone, Copy, Debug)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Number(u8),
}

impl Card {
    fn from_char(c: char) -> Option<Card> {
        match c {
            'A' => Some(Card::A) ,
            'K' => Some(Card::K) ,
            'Q' => Some(Card::Q) ,
            'J' => Some(Card::J) ,
            'T' => Some(Card::T) ,
            _ if c.is_digit(10) => {
                let d = c.to_digit(10).unwrap();
                if d > 1 {
                    Some(Card::Number(d as u8))
                } else {
                    None
                }
            }
            _ => None
        }
    }

    fn strenght(&self) -> u8 {
        if IS_PART2 {
            match self  {
                Card::A => 13,
                Card::K => 12,
                Card::Q => 11,
                Card::J => 0,
                Card::T => 10,
                Card::Number(n) => *n - 1
            }
        } else {
            match self {
                Card::A => 13,
                Card::K => 12,
                Card::Q => 11,
                Card::J => 10,
                Card::T => 9,
                Card::Number(n) => *n - 2
            }
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.strenght() == other.strenght()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Card {
    fn le(&self, other: &Self) -> bool {
        self.strenght() <= other.strenght()
    }

    fn lt(&self, other: &Self) -> bool {
        self.strenght() < other.strenght()
    }

    fn ge(&self, other: &Self) -> bool {
        other.le(self)
    }

    fn gt(&self, other: &Self) -> bool {
        other.lt(self)
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.strenght().partial_cmp(&other.strenght())
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfKind([Card; 5]),
    FourOfKind([Card; 5]),
    FullHouse([Card; 5]),
    ThreeOfKind([Card; 5]),
    TwoPair([Card; 5]),
    OnePair([Card; 5]),
    HighCard([Card;5])
}

impl HandType {
    fn from_string(hand_str: &str) -> Option<HandType> {
        if hand_str.len() != 5 {
            return None;
        }
        let mut hand: [Card; 5] = [Card::A; 5];
        let mut copies: [u8; 14] = [0; 14];
        for (i, character) in hand_str.chars().enumerate(){
            if let Some(card) = Card::from_char(character){
                hand[i] = card;
                copies[card.strenght() as usize] += 1;
            } else {
                return None;
            }
        }
        let mut most_copies = 0;
        let mut num_of_dublets_or_better = 0;

        for (i, cop) in copies.iter().enumerate() {
            if IS_PART2 && i == (Card::J.strenght() as usize){
                continue;
            }
            most_copies = most_copies.max(*cop);
            if *cop > 1 {
                num_of_dublets_or_better += 1;
            }
        }

        if IS_PART2 {
            most_copies += copies[Card::J.strenght() as usize];
        } 

        match (most_copies, num_of_dublets_or_better) {
            (5, _) => Some(HandType::FiveOfKind(hand)),
            (4, _) => Some(HandType::FourOfKind(hand)),
            (3, 2) => Some(HandType::FullHouse(hand)),
            (3, _) => Some(HandType::ThreeOfKind(hand)),
            (2, 2) => Some(HandType::TwoPair(hand)),
            (2, _) => Some(HandType::OnePair(hand)),
            _ => Some(HandType::HighCard(hand))
        }
        
    }

    fn strenght(&self) -> u8 {
        match self {
            HandType::FiveOfKind(_) => 6,
            HandType::FourOfKind(_) => 5,
            HandType::FullHouse(_) => 4,
            HandType::ThreeOfKind(_) => 3,
            HandType::TwoPair(_) => 2,
            HandType::OnePair(_) => 1,
            HandType::HighCard(_) => 0
        }
    }
}

impl HandType {
    fn get_cards(&self) -> &[Card; 5]{
        match self {
            Self::FiveOfKind(cards) => cards,
            Self::FourOfKind(cards) => cards,
            Self::FullHouse(cards) => cards,
            Self::ThreeOfKind(cards) => cards,
            Self::TwoPair(cards) => cards,
            Self::OnePair(cards) => cards,
            Self::HighCard(cards) => cards
        }
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        let hand_ord = (self.strenght()).partial_cmp(&other.strenght()).unwrap();
        match hand_ord {
            Ordering::Equal => {
                let cards1 = self.get_cards();
                let cards2 = self.get_cards();
                for i in 0..5{
                    if cards1[i] != cards2[i] {
                        return  false;
                    }
                }
                true
            },
            _ => false
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hand_ord = (self.strenght()).partial_cmp(&other.strenght()).unwrap();
        match hand_ord {
            Ordering::Equal => {
                let cards1 = self.get_cards();
                let cards2 = other.get_cards();
                for i in 0..5{
                    let cmp = cards1[i].partial_cmp(&cards2[i]).unwrap();
                    if cmp != Ordering::Equal {
                        return  Some(cmp);
                    }
                }
                Some(Ordering::Equal)
            },
            ord => Some(ord)
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => true,
            _ => false
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => true,
            _ => false
        }
    }

    fn le(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Less) | Some(Ordering::Equal) => true,
            _ => false
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => true,
            _ => false
        }
    }
}

fn solution(input: &str) -> u64 {
    let mut hands: Vec<(HandType, u64)> = input.lines().map(|line|{
        (HandType::from_string(line.split_whitespace().next().unwrap()).expect("all lines should have a valid hand"),
         line.split_whitespace().nth(1).unwrap().parse::<u64>().expect("all lines should have a valid bid"))
    }).collect();
    hands.sort_by(|(hand1, _), (hand2, _)| {
        hand1.partial_cmp(hand2).unwrap()
    });
    hands.iter().enumerate().fold(0, |acc, (i, (_, bid))| acc + (*bid) * (i as u64 + 1))
}

fn main() {
    let path = "./input.txt";
    println!("{}", solution(&fs::read_to_string(path).unwrap()));
}
