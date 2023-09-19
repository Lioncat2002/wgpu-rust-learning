mod engine;

fn main() {
    pollster::block_on(engine::entry_point::run())
}
