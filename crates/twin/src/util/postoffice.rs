use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::AddAssign;
use std::sync::Mutex;

static INTERNED_STRINGS: once_cell::sync::Lazy<Mutex<HashMap<String, u32>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));

fn intern_string(s: &str) -> u32 {
    let mut map = INTERNED_STRINGS.lock().unwrap();
    if let Some(&id) = map.get(s) {
        id
    } else {
        let id = map.len() as u32;
        map.insert(s.to_string(), id);
        id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignalId(u32);

impl<T: Into<String>> From<T> for SignalId {
    fn from(value: T) -> Self {
        SignalId(intern_string(&value.into()))
    }
}

pub trait CloneAny: Any {
    fn clone_box(&self) -> Box<dyn CloneAny>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + Clone + 'static> CloneAny for T {
    fn clone_box(&self) -> Box<dyn CloneAny> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct PostOffice {
    current: HashMap<SignalId, Box<dyn CloneAny>>,
    next: HashMap<SignalId, Box<dyn CloneAny>>,
    registry: HashMap<SignalId, TypeId>,
}

impl PostOffice {
    pub fn new() -> Self {
        Self {
            current: HashMap::new(),
            next: HashMap::new(),
            registry: HashMap::new(),
        }
    }

    pub fn register<T: 'static + Clone>(&mut self, id: SignalId) {
        self.registry.insert(id, TypeId::of::<T>());
    }

    pub fn read<T: 'static>(&self, id: SignalId) -> Option<&T> {
        let boxed = self.current.get(&id)?;
        (**boxed).as_any().downcast_ref::<T>()
    }

    pub fn deliver_mail<T: 'static + Clone>(&mut self, id: SignalId, value: T) {
        self.write(id, value);
    }

    pub fn write<T: 'static + Clone>(&mut self, id: SignalId, value: T) {
        if let Some(&expected_type) = self.registry.get(&id) {
            if TypeId::of::<T>() == expected_type {
                self.next.insert(id, Box::new(value));
            } else {
                panic!("Type mismatch for signal {:?}", id);
            }
        }
    }

    pub fn accumulate<T>(&mut self, id: SignalId, value: T)
    where
        T: 'static + AddAssign + Clone + Default,
    {
        if let Some(existing_any) = self.next.get_mut(&id) {
            if let Some(existing_val) = (**existing_any).as_any_mut().downcast_mut::<T>() {
                *existing_val += value;
            }
        } else {
            self.write(id, value);
        }
    }

    pub fn clear_accumulator<T: 'static + Clone + Default>(&mut self, id: SignalId) {
        self.write(id, T::default());
    }

    pub fn flip(&mut self) {
        // 1. Move computed 'next' to 'current'
        self.current = std::mem::take(&mut self.next);

        // 2. Clone 'current' back to 'next' for persistence
        let mut persistent_next = HashMap::new();
        for (id, val) in self.current.iter() {
            // (**val) reaches the dyn CloneAny trait object
            persistent_next.insert(*id, (**val).clone_box());
        }
        self.next = persistent_next;
    }
}
