use game::game::run;

fn main() {
    pollster::block_on(run());
}
