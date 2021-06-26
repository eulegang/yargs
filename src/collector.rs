use std::num::NonZeroU32;

pub struct Collector {
    limit: Limit,
    store: Vec<String>,
}

pub enum Limit {
    Limit(NonZeroU32),
    Unlimited,
}

impl Collector {
    pub fn new(limit: Limit) -> Collector {
        let store = Vec::with_capacity(limit.hint());

        Collector { limit, store }
    }

    pub fn full(&self) -> bool {
        match self.limit {
            Limit::Limit(limit) if self.store.len() as u32 >= limit.into() => true,
            _ => false,
        }
    }

    pub fn push(&mut self, text: String) {
        self.store.push(text);
    }

    pub fn refs(&self) -> Vec<&str> {
        self.store.iter().map(|s| s.as_str()).collect()
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}

impl Limit {
    fn hint(&self) -> usize {
        match self {
            Limit::Unlimited => 64,
            Limit::Limit(limit) => {
                let l: u32 = limit.get();
                l as usize
            }
        }
    }
}

impl From<u32> for Limit {
    fn from(s: u32) -> Limit {
        match s {
            0 => Limit::Unlimited,
            _ => Limit::Limit(unsafe { NonZeroU32::new_unchecked(s) }),
        }
    }
}
