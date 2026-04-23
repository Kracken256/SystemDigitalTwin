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

pub struct PostOffice {
    current: HashMap<SignalId, Box<dyn Any>>,
    next: HashMap<SignalId, Box<dyn Any>>,
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

    fn assert_type_match<T: 'static>(&self, id: SignalId) {
        if let Some(&expected_type) = self.registry.get(&id) {
            if expected_type != TypeId::of::<T>() {
                panic!(
                    "Type mismatch for signal {:?}: expected {:?}, got {:?}",
                    id,
                    expected_type,
                    TypeId::of::<T>()
                );
            }
        } else {
            panic!("Signal {:?} not registered", id);
        }
    }

    pub fn read<T: 'static>(&self, id: SignalId) -> Option<&T> {
        self.assert_type_match::<T>(id);
        self.current.get(&id)?.downcast_ref::<T>()
    }

    pub fn write<T: 'static + Clone>(&mut self, id: SignalId, value: T) {
        self.assert_type_match::<T>(id);
        self.next.insert(id, Box::new(value));
    }

    pub fn accumulate<T>(&mut self, id: SignalId, value: T)
    where
        T: 'static + AddAssign + Clone + Default,
    {
        self.assert_type_match::<T>(id);

        self.next
            .entry(id)
            .and_modify(|existing_any| {
                let existing_val = (**existing_any).downcast_mut::<T>().unwrap();
                *existing_val += value.clone();
            })
            .or_insert_with(|| Box::new(value));
    }

    pub fn clear_accumulator<T: 'static + Clone + Default>(&mut self, id: SignalId) {
        self.write(id, T::default());
    }

    pub fn flip(&mut self) {
        std::mem::swap(&mut self.current, &mut self.next);
        self.next.clear();
    }
}
