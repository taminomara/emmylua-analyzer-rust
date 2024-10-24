#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaVersionNumber {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl LuaVersionNumber {
    #[allow(unused)]
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    #[allow(unused)]
    pub const LUA_JIT: Self = Self {
        major: 2,
        minor: 0,
        patch: 0,
    };

    pub fn from_str(s: &str) -> Option<Self> {
        if s == "JIT" {
            return Some(Self::LUA_JIT);
        }

        let mut iter = s.split('.').map(|it| it.parse::<u32>().unwrap_or(0));
        let major = iter.next().unwrap_or(0);
        let minor = iter.next().unwrap_or(0);
        let patch = iter.next().unwrap_or(0);
        Some(Self {
            major,
            minor,
            patch,
        })
    }
}

impl PartialOrd for LuaVersionNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LuaVersionNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.major
            .cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch))
    }
}
