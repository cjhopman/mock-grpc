// build.rs
use grpc_build::build;

fn main() {
    build("./protos", "src/protogen", true, true, true).unwrap();
}
