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

trait IntoSystem<Input> {
    type System: System;
    fn into_system(self) -> Self::System;
}
impl<F: FnMut()> IntoSystem<()> for F {
    type System = FunctionSystem<Self, ()>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
impl<F: FnMut(T1), T1: 'static> IntoSystem<(T1,)> for F {
    type System = FunctionSystem<Self, (T1,)>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
impl<F: FnMut(T1, T2), T1: 'static, T2: 'static> IntoSystem<(T1, T2)> for F {
    type System = FunctionSystem<Self, (T1, T2)>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
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

impl<F: FnMut(I1, I2), I1: 'static, I2: 'static> System for FunctionSystem<F, (I1, I2)> {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        let i1 = *resources
            .remove(&TypeId::of::<I1>())
            .unwrap()
            .downcast::<I1>()
            .unwrap();
        let i2 = *resources
            .remove(&TypeId::of::<I2>())
            .unwrap()
            .downcast::<I2>()
            .unwrap();
        (self.f)(i1, i2);
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

    // pub fn add_system<S: System + 'static>(&mut self, system: S) -> &mut Self {
    //     self.systems.push(Box::new(system.into_system()));
    //     self
    // }

    pub fn add_system<I, S: System + 'static>(
        &mut self,
        system: impl IntoSystem<I, System = S>,
    ) -> &mut Self {
        self.systems.push(Box::new(system.into_system()));
        self
    }

    pub fn add_resource<R: 'static>(&mut self, resource: R) -> &mut Self {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
        self
    }
}

fn get_i32() -> i32 {
    42
}

fn get_string() -> String {
    "Hello, World!".to_string()
}

fn start_system(i: i32, s: String) {
    println!("i: {}, s: {}", i, s);
}

fn main() {
    Scheduler::new()
        .add_system(start_system)
        .add_resource(get_i32())
        .add_resource(get_string())
        .run();
}
