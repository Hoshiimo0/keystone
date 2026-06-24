// cbindgen で Header を自動生成
// cbindgen --config cbindgen.toml --crate ks-daemon --output ../app/Sources/Bridge/ks_core.h
fn main() {
    println!("cargo:return-if-changed=crates/ks-ipc/src/lib.rs");
}
