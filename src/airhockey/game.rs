//! Airhockey game.

use super::{player::Player, score::Score, ball::Ball};
use alloc::vec::Vec;

const POINTS_PER_GOAL: u8 = 1;
const COLOR_ARRAY: [u32; 4] = [0xfff000, 0xfff000, 0xfff000, 0xfff000];

pub struct Game {
    players: Vec<Player>,
    score: Score,
    ball: Ball
}
fn createGameElements(number_players: u8) -> (Ball, Vec<Player>, Score) {
    let ball = Ball::new();
    let mut players: Vec<Player> = Vec::new();
        for p in 0..number_players {
            players.push(Player::new(p))
        }
    let score = Score::new(players.len() as u8, 10);
    
    return {(ball, players, score)};
    }

impl Game {
    // game constructor
    pub fn new(number_players: u8) -> Game {
        let game_elements =  createGameElements(number_players);

        Game {
            players: game_elements.1,
            score: game_elements.2,
            ball: game_elements.0
        }
    }
    

    // is touched method
    pub fn is_touched(&self, p_id: usize) -> bool {
        self.players[p_id].get_position();
        return false;
    }

    pub fn start(&self, max_score: u16, ball_speed: u16, use_gravity: bool) {
        // self.score = Score::new(self.players.len() as u8,max_score);
        false;
    }
    pub fn game_loop(&self) -> bool {
            // self.handle_inputs();
            // self.handle_physics();
            let scored: u8 = self.evaluate_score();
            if scored != 0 {
                self.score.add_score(self.evaluate_score());
                if self.score.is_game_over().0 {
                    return false;
                }
            }
            return true;
            // self.handle_graphcis();
    }

    fn check_win_condition(&self) -> bool {
        if self.score.is_game_over().0 {
            return true;
        } else {
            return false;
            // print player self.score.is_game_over() has won
        }
    }

    fn handle_inputs(&self) {
        // self.input.handle_gui_inputs(); // pause or other controls
        // self.input.handle_player_inputs(); // handle all player inputs
    }

    fn handle_physics(&self) {
        // self.physics.handle_physics();
    }

    fn evaluate_score(&self) -> u8 {
        return 0;
    }

    fn render(&self) {

        // for p in self.players {
        // self.controller.graphics.draw_circle(COLOR_ARRAY[(p.player_id as usize)], p.get_position().0)
        // }
    }

    fn collisions(&self) {}

    // pub fn init(&self) {

    // }
}
