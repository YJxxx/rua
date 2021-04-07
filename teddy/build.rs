fn main() {
    tonic_build::compile_protos("../proto/teddy/teddy.proto").unwrap();
    tonic_build::compile_protos("../proto/maine/maine.proto").unwrap();
    tonic_build::compile_protos("../proto/message/message.proto").unwrap();
    tonic_build::compile_protos("../proto/ragdoll/ragdoll.proto").unwrap();
}
