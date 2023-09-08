use cucumber::World;
use myworld::AnimalWorld;

mod myworld;
mod steps {
    mod a_givens;
    mod b_whens;
    mod c_thens;
}

fn main() {
    futures::executor::block_on(AnimalWorld::run("tests/features"));
}
