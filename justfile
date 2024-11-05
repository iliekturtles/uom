test-min:
    @echo "Test all crates"
    cargo test --all --verbose --features "use_serde"

test-si:
    @echo "Test si"
    cargo test --verbose --no-default-features --features "f32 si"

test-non-storage:
    @echo "Test all non-storage type features"
    cargo test --verbose --no-default-features --features "autoconvert f32 si use_serde"

test-si-storage:
    @echo "Test si with underlying storage types"
    cargo test --verbose --no-run --no-default-features --features "autoconvert usize isize bigint bigrational complex32 orderedf32 notnanf32 si std use_serde"
    
test-all-non-si:    
    @echo "Test all non-si features"
    cargo test --verbose --no-run --no-default-features --features "autoconvert usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 bigint biguint rational rational32 rational64 bigrational complex32 complex64 orderedf32 orderedf64 notnanf32 notnanf64 f32 f64 std use_serde"
    
test-ci: test-min test-si test-non-storage test-si-storage test-all-non-si