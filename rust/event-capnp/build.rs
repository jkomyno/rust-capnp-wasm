fn main() {
    ::capnpc::CompilerCommand::new()
        .file("../../src/capnp/event.capnp")
        .src_prefix("../../src/capnp")
        .run()
        .expect("compiling schema");
}
