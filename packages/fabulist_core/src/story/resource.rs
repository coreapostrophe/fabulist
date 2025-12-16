use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    rc::Rc,
};

pub trait Keyed {
    fn id(&self) -> &String;
}

#[derive(Debug)]
pub struct Resources(HashMap<TypeId, HashMap<String, Rc<dyn Any>>>);

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

impl Resources {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn get<T>(&self, key: impl Into<String>) -> Option<Rc<T>>
    where
        T: Debug + 'static,
    {
        let key = key.into();
        let type_name = TypeId::of::<T>();
        let resource_map = self.0.get(&type_name);
        let resource = match resource_map {
            Some(resource_map) => resource_map.get(&key),
            None => None,
        };
        match resource {
            Some(resource) => {
                let resource = resource.clone();
                let downcasted_resource = resource.downcast::<T>().unwrap_or_else(|_| {
                    panic!(
                        "Resource map value to match type `{}`.",
                        std::any::type_name::<T>()
                    )
                });
                Some(downcasted_resource)
            }
            None => None,
        }
    }
    pub fn insert<T>(&mut self, resource: T)
    where
        T: Keyed + Clone + 'static,
    {
        self.insert_collection([resource]);
    }
    pub fn insert_collection<T, const N: usize>(&mut self, collection: [T; N])
    where
        T: Keyed + Clone + 'static,
    {
        let mut resource_map: HashMap<String, Rc<dyn Any>> = HashMap::new();
        collection.iter().for_each(|res| {
            resource_map.insert(res.id().clone(), Rc::new(res.to_owned()));
        });
        self.0.insert(TypeId::of::<T>(), resource_map);
    }
}

#[derive(Debug)]
pub struct Inset<T>(String, Option<Rc<T>>);

impl<T> Inset<T> {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into(), None)
    }
    pub fn id(&self) -> &String {
        &self.0
    }
    pub fn set_id(&mut self, id: impl Into<String>) {
        self.0 = id.into();
    }
    pub fn value(&self) -> Option<&Rc<T>> {
        self.1.as_ref()
    }
    pub fn set_value(&mut self, value: Option<Rc<T>>) {
        self.1 = value;
    }
}

pub trait InterpInset {
    fn interp_inset(&mut self, resource: &mut Resources);
}
