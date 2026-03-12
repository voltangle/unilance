use built;

fn main() {
    built::write_built_file().expect("Should be able to acquire build-time info");
}
