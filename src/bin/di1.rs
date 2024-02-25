use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

struct FunctionSystem<F, Input> {
    f: F,
    marker: std::marker::PhantomData<fn() -> Input>,
}

trait System {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>);
}

type StoredSystem = Box<dyn System>;
struct Scheduler {
    systems: Vec<StoredSystem>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl<F: FnMut()> System for FunctionSystem<F, ()> {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        (self.f)();
    }
}

impl<F: FnMut(I1), I1: 'static> System for FunctionSystem<F, (I1,)> {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        let i1 = *resources
            .remove(&TypeId::of::<I1>())
            .unwrap()
            .downcast::<I1>()
            .unwrap();
        (self.f)(i1);
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
            resources: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        for system in self.systems.iter_mut() {
            system.run(&mut self.resources);
        }
    }

    pub fn add_resource<R: 'static>(&mut self, resource: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
    }
}

fn main() {
    println!("list: {:?}", 2); // Prints "not a match"
}
