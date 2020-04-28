pub mod event;

use std::collections::BTreeMap;
use tui::widgets::ListState;

pub struct StatefulHash<T, V> {
    pub state: ListState,
    pub items: BTreeMap<T, V>,
}

impl<T, V> StatefulHash<T, V> {
    pub fn with_items(items: BTreeMap<T, V>) -> StatefulHash<T, V> {
        StatefulHash {
            state: ListState::default(),
            items: items,
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: items,
        }
    }
}