pub fn budget(ram: u64, cache: u64) -> u64 {
    cache.clamp(16, 512).min((ram / 8).max(16))
}
