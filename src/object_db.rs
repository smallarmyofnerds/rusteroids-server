use std::{cell::RefCell, collections::HashMap};

use crate::{id_generator::IdGenerator, object::Object};

pub struct DbObject {
    id: u64,
    object: RefCell<Box<dyn Object>>,
}

impl DbObject {
    pub fn new(id: u64, object: RefCell<Box<dyn Object>>) -> Self {
        Self { id, object }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_object(&self) -> &RefCell<Box<dyn Object>> {
        &self.object
    }
}

pub struct ObjectDb {
    id_generator: IdGenerator,
    objects: Vec<DbObject>,
    objects_by_id: HashMap<u64, usize>,
}

impl ObjectDb {
    pub fn new() -> Self {
        Self {
            id_generator: IdGenerator::new(),
            objects: vec![],
            objects_by_id: HashMap::new(),
        }
    }

    pub fn add(&mut self, o: Box<dyn Object>) {
        let id = self.id_generator.get_next_id();
        self.objects.push(DbObject::new(id, RefCell::new(o)));
        self.objects_by_id.insert(id, self.objects.len() - 1);
    }

    pub fn for_each(&mut self, f: impl Fn(&DbObject)) {
        for o in &mut self.objects {
            f(o);
        }
    }

    fn reindex(&mut self) {
        self.objects_by_id.clear();
        for (i, o) in self.objects.iter().enumerate() {
            self.objects_by_id.insert(o.get_id(), i);
        }
    }

    pub fn remove_deleted(&mut self) {
        self.objects
            .retain(|o| !o.get_object().borrow().is_flagged_for_destruction());
        self.reindex();
    }

    pub fn all(&self) -> impl Iterator<Item = &DbObject> {
        self.objects.iter()
    }

    pub fn all_where(&self, f: impl Fn(&&DbObject) -> bool) -> impl Iterator<Item = &DbObject> {
        self.objects.iter().filter(f)
    }

    pub fn count_where(&self, f: impl Fn(&&DbObject) -> bool) -> usize {
        self.objects.iter().filter(f).count()
    }

    pub fn get(&self, id: u64) -> &DbObject {
        &self.objects[self.objects_by_id[&id]]
    }
}
