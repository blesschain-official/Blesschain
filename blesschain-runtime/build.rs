fn main() {
    #[cfg(feature = "std")]
    {
        let wasm_binary = std::fs::read("blesschain_runtime.compact.wasm")
            .expect("WASM file should exist in standard builds");
        let out_dir = std::env::var("OUT_DIR").unwrap();
        std::fs::write(std::path::Path::new(&out_dir).join("wasm_binary.rs"), format!(
            "pub const WASM_BINARY: Option<&[u8]> = Some(&{:?});",
            wasm_binary
        )).unwrap();
    }
}
