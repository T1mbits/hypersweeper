//! Trait definition for [`Event`] as well as [`EventDispatcher`] and its default methods

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

/// Represents an event subscriber
pub(crate) type Subscriber<T> = Arc<dyn Fn(T) + Send + Sync>;

/// Types representing events
pub trait Event: Send + Sync + 'static {
    /// Data or context associated with an event
    type Context: Send + Sync + Clone;
}

/// A struct holding event subscribers and emitting events
#[derive(Default)]
pub struct EventDispatcher {
    /// Hashmap used to store [`Event`]s and their corresponding subscribers
    subscribers: HashMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
}

impl EventDispatcher {
    /// Create an empty [`EventDispatcher`]
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    /// Register a callback for a given [`Event`]
    pub fn subscribe<E: Event>(&mut self, callback: Subscriber<E::Context>) {
        let entry = self.subscribers.entry(TypeId::of::<E>()).or_default();
        entry.push(Box::new(callback))
    }

    /// Emit an [`Event`] with the associated context type
    pub(crate) fn emit<E: Event>(&self, context: E::Context) {
        if let Some(subscribers) = self.subscribers.get(&TypeId::of::<E>()) {
            for subscriber in subscribers {
                if let Some(callback) = subscriber.downcast_ref::<Subscriber<E::Context>>() {
                    callback(context.clone());
                }
            }
        }
    }
}
