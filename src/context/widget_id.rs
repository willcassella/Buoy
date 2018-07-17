use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct WidgetId {
    id: u64,
}

impl WidgetId {
    pub fn str(id: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);

        WidgetId {
            id: hasher.finish(),
        }
    }

    pub fn num(id: u64) -> Self {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);

        WidgetId {
            id: hasher.finish(),
        }
    }

    pub fn prefix(prefix: WidgetId, id: WidgetId) -> Self {
        let mut hasher = DefaultHasher::new();
        prefix.hash(&mut hasher);
        id.hash(&mut hasher);

        WidgetId {
            id: hasher.finish(),
        }
    }

    pub fn prefix_str(prefix: WidgetId, id: &str) -> Self {
        WidgetId::prefix(prefix, WidgetId::str(id))
    }
}