#[derive(Debug)]
pub struct CacheOptions {
    #[allow(unused)]
    pub analysis_phase: bool,
}

impl Default for CacheOptions {
    fn default() -> Self {
        Self {
            analysis_phase: true,
        }
    }
}
