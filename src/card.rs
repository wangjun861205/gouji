#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Suit {
    None,
    Club,
    Diamonds,
    Heart,
    Spade,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Rank {
    _3 = 0,
    _4 = 1,
    _5 = 2,
    _6 = 3,
    _7 = 4,
    _8 = 5,
    _9 = 6,
    _10 = 7,
    _J = 8,
    _Q = 9,
    _K = 10,
    _A = 11,
    _2 = 12,
    _BJK = 13,
    _RJK = 14,
}

#[derive(Clone, Eq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
    fn ne(&self, other: &Self) -> bool {
        self.rank != other.rank
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

#[derive(PartialEq)]
pub struct CardSet(Vec<Card>);

impl CardSet {
    pub fn new(cards: &[Card]) -> Option<Self> {
        if cards.is_empty() {
            return None;
        }
        let mut cards = cards.to_vec();
        cards.sort();
        if (cards[0].rank == Rank::_3 || cards[0].rank == Rank::_4) && cards.last().unwrap().rank > Rank::_A {
            return None;
        }
        for i in 1..cards.len() {
            if cards[i].rank != cards[i - 1].rank {
                if cards[i].rank < Rank::_2 {
                    return None;
                }
                if cards[i].rank == Rank::_2 {
                    cards[i].rank = cards[i - 1].rank;
                }
            }
        }
        Some(Self(cards))
    }

    pub fn from_agg(agg: &[(Suit, Rank, i32)]) -> Option<Self> {
        let cards = agg.iter().fold(Vec::new(), |mut l, (s, r, c)| {
            for _ in 0..*c {
                l.push(Card::new(s.clone(), r.clone()));
            }
            l
        });
        Self::new(&cards)
    }

    pub fn is_gouji(&self) -> bool {
        match self.0[0].rank {
            Rank::_RJK => false,
            Rank::_BJK => true,
            Rank::_2 => true,
            Rank::_A | Rank::_K => self.0.len() > 1,
            Rank::_Q => self.0.len() > 2,
            Rank::_J => self.0.len() > 3,
            Rank::_10 => self.0.len() > 4,
            _ => {
                for c in &self.0 {
                    if c.rank == Rank::_BJK {
                        return true;
                    }
                }
                false
            }
        }
    }

    pub fn is_greater_than(&self, other: &Self) -> bool {
        if self.0.len() - other.0.len() == 1 {
            if other.0.last().unwrap().rank != Rank::_RJK {
                return false;
            }
            if self.0.last().unwrap().rank != Rank::_RJK || self.0[self.0.len() - 2].rank != Rank::_RJK {
                return false;
            }
            for i in 0..self.0.len() - 2 {
                if self.0[i] <= other.0[i] {
                    return false;
                }
            }
            return true;
        }
        for i in 0..self.0.len() {
            if self.0[i] <= other.0[i] {
                return false;
            }
        }
        true
    }
}

#[derive(Clone)]
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new(mut cards: Vec<Card>) -> Self {
        cards.sort();
        Self(cards)
    }

    pub fn from_agg(agg: Vec<(Suit, Rank, i32)>) -> Self {
        let mut cards = agg.into_iter().fold(Vec::new(), |mut l, (s, r, c)| {
            for _ in 0..c {
                l.push(Card::new(s, r));
            }
            l
        });
        cards.sort();
        Self(cards)
    }

    pub fn subtract(&mut self, set: &CardSet) -> bool {
        let mut remain = self.0.clone();
        let mut indices = Vec::new();
        let mut i = 0;
        let mut j = 0;
        'outer: while j < set.0.len() {
            while i < remain.len() {
                if remain[i] == set.0[j] {
                    indices.push(i);
                    i += 1;
                    j += 1;
                    continue 'outer;
                }
                i += 1;
            }
            return false;
        }
        for (i, idx) in indices.into_iter().enumerate() {
            remain.remove(idx - i);
        }
        if set.0[0].rank == Rank::_3 && !remain.is_empty() {
            return false;
        }
        if set.0[0].rank == Rank::_4 && remain.iter().any(|c| c.rank == Rank::_4) {
            return false;
        }
        self.0 = remain;
        true
    }

    pub fn to_vec(self) -> Vec<Card> {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_card_set_is_greater_than() {
        let set1 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_4), Card::new(Suit::Diamonds, Rank::_4)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_3), Card::new(Suit::Diamonds, Rank::_3)]).unwrap();
        assert!(set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_5), Card::new(Suit::None, Rank::_BJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_3), Card::new(Suit::Diamonds, Rank::_3)]).unwrap();
        assert!(set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_5), Card::new(Suit::None, Rank::_RJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_3), Card::new(Suit::Diamonds, Rank::_3)]).unwrap();
        assert!(set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_5), Card::new(Suit::None, Rank::_RJK), Card::new(Suit::None, Rank::_RJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_3), Card::new(Suit::Diamonds, Rank::_3)]).unwrap();
        assert!(!set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_6), Card::new(Suit::None, Rank::_RJK), Card::new(Suit::None, Rank::_RJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_5), Card::new(Suit::None, Rank::_RJK)]).unwrap();
        assert!(set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_6), Card::new(Suit::None, Rank::_BJK), Card::new(Suit::None, Rank::_RJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_5), Card::new(Suit::None, Rank::_RJK)]).unwrap();
        assert!(!set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_6), Card::new(Suit::None, Rank::_BJK), Card::new(Suit::None, Rank::_BJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_5), Card::new(Suit::None, Rank::_RJK)]).unwrap();
        assert!(!set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_6), Card::new(Suit::None, Rank::_BJK), Card::new(Suit::None, Rank::_BJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_5), Card::new(Suit::None, Rank::_BJK)]).unwrap();
        assert!(!set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_6), Card::new(Suit::Heart, Rank::_2), Card::new(Suit::None, Rank::_BJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_5), Card::new(Suit::Heart, Rank::_5), Card::new(Suit::Diamonds, Rank::_5)]).unwrap();
        assert!(set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_6), Card::new(Suit::Heart, Rank::_2), Card::new(Suit::None, Rank::_BJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_5), Card::new(Suit::Heart, Rank::_5), Card::new(Suit::None, Rank::_BJK)]).unwrap();
        assert!(!set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_6), Card::new(Suit::Heart, Rank::_2), Card::new(Suit::None, Rank::_RJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_5), Card::new(Suit::Heart, Rank::_5), Card::new(Suit::None, Rank::_BJK)]).unwrap();
        assert!(set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![
            Card::new(Suit::Club, Rank::_6),
            Card::new(Suit::Heart, Rank::_2),
            Card::new(Suit::None, Rank::_RJK),
            Card::new(Suit::None, Rank::_RJK),
        ])
        .unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_5), Card::new(Suit::Heart, Rank::_5), Card::new(Suit::None, Rank::_BJK)]).unwrap();
        assert!(!set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![
            Card::new(Suit::Club, Rank::_6),
            Card::new(Suit::Heart, Rank::_2),
            Card::new(Suit::None, Rank::_RJK),
            Card::new(Suit::None, Rank::_RJK),
        ])
        .unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_5), Card::new(Suit::Heart, Rank::_5), Card::new(Suit::None, Rank::_RJK)]).unwrap();
        assert!(set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_2), Card::new(Suit::None, Rank::_BJK), Card::new(Suit::None, Rank::_RJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_2), Card::new(Suit::Heart, Rank::_2), Card::new(Suit::Diamonds, Rank::_2)]).unwrap();
        assert!(!set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_2), Card::new(Suit::None, Rank::_BJK), Card::new(Suit::None, Rank::_RJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_A), Card::new(Suit::Heart, Rank::_2), Card::new(Suit::Diamonds, Rank::_2)]).unwrap();
        assert!(set1.is_greater_than(&set2));
        let set1 = CardSet::new(&vec![Card::new(Suit::None, Rank::_BJK), Card::new(Suit::None, Rank::_RJK)]).unwrap();
        let set2 = CardSet::new(&vec![Card::new(Suit::Club, Rank::_2), Card::new(Suit::Heart, Rank::_2)]).unwrap();
        assert!(set1.is_greater_than(&set2));
    }

    #[test]
    fn test_is_gouji() {
        let set = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_3), Card::new(Suit::Heart, Rank::_3)]).unwrap();
        assert!(!set.is_gouji());
        let set = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_10); 4]).unwrap();
        assert!(!set.is_gouji());
        let set = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_10); 5]).unwrap();
        assert!(set.is_gouji());
        let set = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_J); 3]).unwrap();
        assert!(!set.is_gouji());
        let set = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_J); 4]).unwrap();
        assert!(set.is_gouji());
        let set = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_Q); 2]).unwrap();
        assert!(!set.is_gouji());
        let set = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_Q); 3]).unwrap();
        assert!(set.is_gouji());
        let set = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_K); 1]).unwrap();
        assert!(!set.is_gouji());
        let set = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_K); 2]).unwrap();
        assert!(set.is_gouji());
        let set = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_A); 1]).unwrap();
        assert!(!set.is_gouji());
        let set = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_A); 2]).unwrap();
        assert!(set.is_gouji());
        let set = CardSet::new(&vec![Card::new(Suit::Heart, Rank::_10); 4]).unwrap();
        assert!(!set.is_gouji());
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_10, 3), (Suit::Heart, Rank::_2, 1)]).unwrap();
        assert!(!set.is_gouji());
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_10, 4), (Suit::Heart, Rank::_2, 1)]).unwrap();
        assert!(set.is_gouji());
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_J, 2), (Suit::Heart, Rank::_2, 1)]).unwrap();
        assert!(!set.is_gouji());
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_J, 3), (Suit::Heart, Rank::_2, 1)]).unwrap();
        assert!(set.is_gouji());
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_Q, 1), (Suit::Heart, Rank::_2, 1)]).unwrap();
        assert!(!set.is_gouji());
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_Q, 2), (Suit::Heart, Rank::_2, 1)]).unwrap();
        assert!(set.is_gouji());
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_K, 1), (Suit::Heart, Rank::_2, 1)]).unwrap();
        assert!(set.is_gouji());
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_A, 1), (Suit::Heart, Rank::_2, 1)]).unwrap();
        assert!(set.is_gouji());
        let set = CardSet::from_agg(&vec![(Suit::None, Rank::_BJK, 1)]).unwrap();
        assert!(set.is_gouji());
        let set = CardSet::from_agg(&vec![(Suit::None, Rank::_RJK, 1)]).unwrap();
        assert!(!set.is_gouji());
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_2, 1)]).unwrap();
        assert!(set.is_gouji());
        let set = CardSet::from_agg(&vec![(Suit::None, Rank::_BJK, 1), (Suit::Heart, Rank::_5, 1)]).unwrap();
        assert!(set.is_gouji());
    }

    #[test]
    fn test_subtract() {
        let mut hand = Hand::from_agg(vec![(Suit::Heart, Rank::_3, 1), (Suit::Heart, Rank::_4, 1)]);
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_3, 1)]).unwrap();
        assert!(!hand.subtract(&set));
        let mut hand = Hand::from_agg(vec![(Suit::Heart, Rank::_3, 1), (Suit::Heart, Rank::_4, 1)]);
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_4, 1)]).unwrap();
        assert!(hand.subtract(&set));
        assert!(hand.to_vec() == vec![Card { suit: Suit::Heart, rank: Rank::_3 }]);
        let mut hand = Hand::from_agg(vec![(Suit::Heart, Rank::_3, 1), (Suit::Heart, Rank::_4, 2)]);
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_4, 1)]).unwrap();
        assert!(!hand.subtract(&set));
        let mut hand = Hand::from_agg(vec![(Suit::Heart, Rank::_3, 1), (Suit::Heart, Rank::_4, 2)]);
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_4, 2)]).unwrap();
        assert!(hand.subtract(&set));
        assert!(hand.to_vec() == vec![Card { suit: Suit::Heart, rank: Rank::_3 }]);
        let mut hand = Hand::from_agg(vec![(Suit::Heart, Rank::_3, 1), (Suit::Heart, Rank::_4, 2)]);
        let set = CardSet::from_agg(&vec![(Suit::Heart, Rank::_4, 3)]).unwrap();
        assert!(!hand.subtract(&set));
    }
}
