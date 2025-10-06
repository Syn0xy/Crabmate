use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone)]
pub struct BijectionMap<A, B>
where
    A: Eq + Hash + Clone,
    B: Eq + Hash + Clone,
{
    forward: HashMap<A, B>,
    backward: HashMap<B, A>,
}

impl<A, B> Default for BijectionMap<A, B>
where
    A: Eq + Hash + Clone,
    B: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self {
            forward: Default::default(),
            backward: Default::default(),
        }
    }
}

impl<A, B> BijectionMap<A, B>
where
    A: Eq + Hash + Clone,
    B: Eq + Hash + Clone,
{
    pub fn insert(&mut self, a: A, b: B) {
        // Supprimer les anciens liens s'ils existent
        if let Some(old_backward) = self.forward.insert(a.clone(), b.clone()) {
            self.backward.remove(&old_backward);
        }
        if let Some(old_forward) = self.backward.insert(b.clone(), a.clone()) {
            self.forward.remove(&old_forward);
        }
    }

    pub fn remove_by_forward(&mut self, a: &A) {
        if let Some(value) = self.forward.remove(a) {
            self.backward.remove(&value);
        }
    }

    pub fn remove_by_backward(&mut self, b: &B) {
        if let Some(value) = self.backward.remove(b) {
            self.forward.remove(&value);
        }
    }

    pub fn get_by_forward(&self, a: &A) -> Option<&B> {
        self.forward.get(a)
    }

    pub fn get_by_backward(&self, b: &B) -> Option<&A> {
        self.backward.get(b)
    }

    pub fn contains_key_forward(&self, a: &A) -> bool {
        self.forward.contains_key(a)
    }

    pub fn contains_key_backward(&self, b: &B) -> bool {
        self.backward.contains_key(b)
    }

    pub fn values_forward(&self) -> impl Iterator<Item = &B> {
        self.forward.values()
    }

    pub fn values_backward(&self) -> impl Iterator<Item = &A> {
        self.backward.values()
    }
}
