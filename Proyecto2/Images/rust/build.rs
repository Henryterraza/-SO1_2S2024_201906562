fn main() {
    tonic_build::compile_protos("src/discipline.proto").unwrap();
}