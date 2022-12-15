pub enum Suit {
    Club,
    Diamonds,
    Heart,
    Spade,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

pub struct Card {
    suit: Suit,
    rank: Rank,
}

pub struct CardSet {
    base_rank: Rank,
    base_count: i32,
    money_count: i32,
    black_joker_count: i32,
    red_joker_count: i32,
}

impl CardSet {
    pub fn new(cards: &mut [Card]) -> Result<Self, String> {
        if cards.is_empty() {
            return Err("牌组不得为空".into());
        }
        cards.sort_by_key(|c| c.rank);
	let mut res = Self {
		base_rank: if cards[0].rank < 12 { cards[0].rank }
		base_count: 1,
		money_count: 0,
		black_joker_count: 0,
		red_joker_count: 0,
	}
        for c in cards.into_iter().skip(1) {
		if c.rank == res.base_rank {
			res.base_count += 1;
			continue;
		}
	}
    }
}
