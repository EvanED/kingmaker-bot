use cucumber::World;
//use cucumber::{writer, WriterExt};
//use std::io;
use context::TestContext;

mod context;
mod steps {
    mod a_givens;
    mod b_whens;
    mod c_thens;
}

fn main() {
    futures::executor::block_on(
        TestContext::cucumber()
        //.with_writer(
        //    writer::Basic::raw(io::stdout(), writer::Coloring::Never, 0)
        //        .summarized()
        //        .assert_normalized(),
        //)
        .run("tests/features"));
}
