pub fn generate_short_code(uid: &u64) -> String {
    base62::encode(*uid)
}
