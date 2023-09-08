use cucumber::World;
use context::AnimalWorld;

mod context;
mod steps {
    mod a_givens;
    mod b_whens;
    mod c_thens;
}

fn main() {
    futures::executor::block_on(AnimalWorld::run("tests/features"));
}
