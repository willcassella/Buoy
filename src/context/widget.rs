use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use tree::Filter;

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

pub struct WidgetInfo {
    pub id: WidgetId,
    pub inner_filters: Vec<Rc<Filter>>,
}

impl WidgetInfo {
    pub fn new(id: WidgetId) -> Self {
        WidgetInfo {
            id,
            inner_filters: Vec::new(),
        }
    }

    pub fn new_str_id(id: &str) -> Self {
        WidgetInfo {
            id: WidgetId::str(id),
            inner_filters: Vec::new(),
        }
    }

    pub fn add_filter(&mut self, filter: Rc<Filter>) {
        self.inner_filters.push(filter);
    }
}