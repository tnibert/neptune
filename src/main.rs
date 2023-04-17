extern crate mercurylib;

use mercurylib::game::Game;
use mercurylib::desktopplatform::DesktopPlatform;
use mercurylib::platform::Platform;

fn main() {
    let mut game = Game::new();
    let mut pf = DesktopPlatform::new();
    pf.gameloop(&mut game);
}


// some notes:
//
// Just remember that 32, Some(32), and None can all be passed into a function whose type implements Into<Option<i32>>.
// This pattern is a relatively easy way to implement optional/default arguments in Rust.
//
// copy_ex() is like copy() but with some more options
// mem::replace() can swap values of same type
//
// input example: https://github.com/PistonDevelopers/piston-examples/blob/master/examples/user_input/src/main.rs