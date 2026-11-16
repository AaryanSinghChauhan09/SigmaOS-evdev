// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-evdev: evdev component model.
//!
//! Real implementation: evdev_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvdevItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvdevError {
    EmptyEvdevItemName,
    DuplicateEvdevItem(String),
    EmptyEventSubject,
}

impl fmt::Display for EvdevError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEvdevItemName => write!(f, "evdev_item name must not be empty"),
            Self::DuplicateEvdevItem(id) => write!(f, "duplicate evdev_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for EvdevError {}

pub struct Evdev {
    pub name: String,
    evdev_items: BTreeMap<String, EvdevItem>,
    events: BTreeMap<String, Event>,
}

impl Evdev {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            evdev_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_evdev_item(&mut self, item: EvdevItem) -> Result<(), EvdevError> {
        if item.name.trim().is_empty() {
            return Err(EvdevError::EmptyEvdevItemName);
        }
        let id = item.id.clone();
        if self.evdev_items.contains_key(&id) {
            return Err(EvdevError::DuplicateEvdevItem(id.clone()));
        }
        self.evdev_items.insert(id, item);
        Ok(())
    }

    pub fn evdev_item_count(&self) -> usize {
        self.evdev_items.len()
    }

    pub fn get_evdev_item(&self, id: &str) -> Option<&EvdevItem> {
        self.evdev_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), EvdevError> {
        if event.subject.trim().is_empty() {
            return Err(EvdevError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_evdev_item(id: &str, name: &str) -> EvdevItem {
        EvdevItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Evdev::new("Evdev");
        assert_eq!(m.name, "Evdev");
        assert_eq!(m.evdev_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_evdev_item() {
        let mut m = Evdev::new("Evdev");
        m.add_evdev_item(sample_evdev_item("a1", "Alpha")).unwrap();
        assert_eq!(m.evdev_item_count(), 1);
        assert!(m.get_evdev_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Evdev::new("Evdev");
        assert!(m.add_evdev_item(sample_evdev_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Evdev::new("Evdev");
        m.add_evdev_item(sample_evdev_item("x", "One")).unwrap();
        assert!(m.add_evdev_item(sample_evdev_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Evdev::new("Evdev");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
