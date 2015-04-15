extern crate retris;

fn main(){
    let mut game = retris::Game::new(800,600);
    game.start();
}
