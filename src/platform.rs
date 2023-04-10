use crate::game::Game;

/*
 * trait to hide platform implementation behind
 */
pub trait Platform {
    fn gameloop(&mut self, game: &mut Game);
}