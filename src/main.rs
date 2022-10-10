use cube_commander::game::init_game;
fn main() {
    pollster::block_on(init_game());
}