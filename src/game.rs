use indexmap::{IndexMap, IndexSet, indexset, indexmap};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct PlayerId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayersGatheringGame {
    players: IndexSet<PlayerId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoosingThemeGiverGame {
    players: IndexSet<PlayerId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaitingForThemeGame {
    players: IndexSet<PlayerId>,
    theme_giver: PlayerId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaitingForAssociationsGame {
    players: IndexSet<PlayerId>,
    theme_giver: PlayerId,
    theme: String,
    words: Vec<IndexSet<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewingAssociationsGame {
    players: IndexSet<PlayerId>,
    theme_giver: PlayerId,
    theme: String,
    words: IndexMap<String, Vec<PlayerId>>,
    banned_words: IndexSet<String>,
}

pub fn create_game(host: PlayerId) -> PlayersGatheringGame {
    PlayersGatheringGame { players: indexset!{host} }
}

impl PlayersGatheringGame {
    pub fn join(&mut self, player: PlayerId) {
        self.players.insert(player);
    }
}

impl ChoosingThemeGiverGame {
    pub fn choose_theme_giver(self, player: PlayerId) -> WaitingForThemeGame {
        WaitingForThemeGame { players: self.players, theme_giver: player }
    }
}

impl WaitingForThemeGame {
    pub fn give_theme(self, theme: String) -> WaitingForAssociationsGame {
        let mut words = Vec::<IndexSet<String>>::new();
        words.resize_with(self.players.len(), Default::default);

        WaitingForAssociationsGame {
            players: self.players,
            theme: theme,
            theme_giver: self.theme_giver,
            words: words,
        }
    }
}

impl WaitingForAssociationsGame {
    pub fn give_association(&mut self, word: String, author: PlayerId) {
        if let Some(authors_idx) = self.players.get_index_of(&author) {
            self.words[authors_idx].insert(word);
        }
    }

    pub fn proceed_to_review(self) -> ReviewingAssociationsGame {
        let mut words_dict = IndexMap::<String, Vec<PlayerId>>::new();
        for (i, ws) in self.words.into_iter().enumerate() {
            for w in ws {
                words_dict.entry(w).or_default().push(self.players[i].to_owned())
            }
        }

        ReviewingAssociationsGame {
            players: self.players,
            theme: self.theme,
            theme_giver: self.theme_giver,
            words: words_dict,
            banned_words: Default::default(),
        }
    }
}

impl ReviewingAssociationsGame {
    pub fn ban(&mut self, word: String) {
        self.banned_words.insert(word);
    }

    pub fn score(&self) -> IndexMap<PlayerId, usize> {
        let it = self.players.iter().map(|x| (x.to_owned(), 0usize));
        let mut res = IndexMap::<PlayerId, usize>::from_iter(it);

        for (word, authors) in &self.words {
            if !self.banned_words.contains(word) {
                for author in authors {
                    *res.get_mut(author).unwrap() += authors.len();
                }
            }
        }
        res
    }
} 

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Game {
    PlayersGathering(PlayersGatheringGame),
    ChoosingThemeGiver(ChoosingThemeGiverGame),
    WaitingForTheme(WaitingForThemeGame),
    GivingAssociations(WaitingForAssociationsGame),
    ReviewingAssociations(ReviewingAssociationsGame),
    Finished,
}