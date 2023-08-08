use maat_rust_binding::bindings;

fn main() {
    unsafe {
        let _opts = bindings::maat_options_new();
    }
}
