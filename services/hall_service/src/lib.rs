pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use std::collections::HashMap;

struct Player {
    name: String,
}
impl Player {
    fn new(name:&str) -> Player {
        Self {
            name: name.to_string(),
        }
    }
}
struct PlayerManager {
    players: HashMap<i64, Player>
}

struct State {
    player_manager: PlayerManager
}

impl PlayerManager {
    fn new() -> PlayerManager {
        Self {
            players: HashMap::new(),
        }
    }
    fn add_player(&mut self, id:i64, player: Player) {
        println!("Adding player {}", id);
        self.players.insert(id, player);
    }
    fn remove_player(&mut self, id:i64) {
        println!("Removing player {}", id);
        self.players.remove(&id);
    }
}

impl State {
    fn new() -> Box<State> {
        Box::new(State {
            player_manager:PlayerManager::new(),
        })
    }
    fn add_player(&mut self, id: i64, player_name: &str) {
        let player = Player::new(player_name);
        self.player_manager.add_player(id, player);
    }
    fn remove_player(&mut self, id: i64) {
        self.player_manager.remove_player(id);
    }
}


pub async fn main() {
    let mut state = State::new();
    let mut index = 1;
    loop {
        println!("Hello, world!");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        match index {
            1=>{
                state.add_player(1, "ljw")
            }
            2=>{
                state.remove_player(1)
            }
            _=>{

            }

        };
        index+=1;
    }
}