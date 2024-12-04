#[no_mangle]
pub extern "C" fn fact(n: u64) -> u64 {
    (1..=n).fold(1, |acc, x| acc * x)
}
