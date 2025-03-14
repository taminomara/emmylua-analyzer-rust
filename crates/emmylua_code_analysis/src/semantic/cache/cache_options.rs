#[derive(Debug)]
pub struct CacheOptions {
    pub allow_cache_members: bool,
}

impl Default for CacheOptions {
    fn default() -> Self {
        Self {
            allow_cache_members: true,
        }
    }
}
