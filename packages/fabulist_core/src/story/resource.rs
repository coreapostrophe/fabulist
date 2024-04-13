use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

use super::traits::Keyed;

#[derive(Debug)]
pub struct Resources(HashMap<TypeId, HashMap<String, Box<dyn Any>>>);

impl Resources {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn get<T>(&self, key: impl Into<String>) -> Option<&T>
    where
        T: 'static,
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
                let downcasted_resource = resource
                    .downcast_ref::<T>()
                    .expect(&format!("Resource map value to match type `{}`.", key));
                Some(downcasted_resource)
            }
            None => None,
        }
    }
    pub fn get_mut<T>(&mut self, key: impl Into<String>) -> Option<&mut T>
    where
        T: 'static,
    {
        let key = key.into();
        let type_name = TypeId::of::<T>();
        let resource_map = self.0.get_mut(&type_name);
        let resource = match resource_map {
            Some(resource_map) => resource_map.get_mut(&key),
            None => None,
        };
        match resource {
            Some(resource) => {
                let downcasted_resource = resource
                    .downcast_mut::<T>()
                    .expect(&format!("Resource map value to match type `{}`.", key));
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
        let mut resource_map: HashMap<String, Box<dyn Any>> = HashMap::new();
        collection.iter().for_each(|res| {
            resource_map.insert(res.id().clone(), Box::new(res.to_owned()));
        });
        self.0.insert(TypeId::of::<T>(), resource_map);
    }
}
