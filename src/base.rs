use rand::thread_rng;
use rand::seq::SliceRandom;

mod cards;
use cards::*;

mod agent;
use agent::*;

#[derive(Debug)]
pub struct DosGame<'a> {
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
    players: Vec<Player<'a>>,
    current_turn: u8,
}


// Need a server representation and a client represention...
// What should be shared vs distinct?
impl<'a> DosGame<'a> {

    pub fn turn(&mut self, turn: Turn) {
        match turn {
            Turn::Draw => {}
            Turn::PlayCard{card} => {}
        }
    }

    pub fn deal_in_players(&mut self) {
        for _ in 0..7 {
            for pid in 0..self.players.len( ) {
                self.deal_card(pid);
            }
        }

        // Give notice to player to start turn? return some sort of struct?
    }

    pub fn deal_card(&mut self, player_id: usize) {
        let card = self.draw_card();
        self.players[player_id].hand.push(card);
    }


    fn draw_card(&mut self) -> Card {

        if let Some(card) = self.deck.pop() {
            card
        } else {
            self.reshuffle_discard_pile();

            if let Some(card) = self.deck.pop() {
                card
            } else {
                // TODO: Handle this case gracefully, maybe return an option instead
                panic!("No cards left")
            }
        }
    }

    pub fn reshuffle_discard_pile(&mut self) {
        self.deck = self.discard_pile.clone();
        self.discard_pile = Vec::new();

        self.deck.shuffle(&mut thread_rng());
    }
}

// TODO: Implment this method
pub fn can_play(top_card: Card, played_card: Card) -> bool {
    return true
}

pub enum Turn {
    Draw,
    PlayCard {
        card: Card,
    },
}

// Should players be initialized and dealt cards?  yes?
pub fn new_game<'a>(num_players: u8) -> DosGame<'a> {

    let deck = new_deck();

    let mut players = Vec::new();
    for i in 0..num_players {
        players.push(Player {
            id: i,
            hand: Vec::new(),
            agent: &Bot{}
        })
    }

    DosGame {
        deck,
        discard_pile: Vec::new(),
        players,
        current_turn: 0,
    }
}




