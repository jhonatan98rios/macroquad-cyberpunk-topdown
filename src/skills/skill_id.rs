use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkillId(usize);

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

impl SkillId {
    pub fn new() -> Self {
        SkillId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}