use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

struct FunctionSystem<F, Input> {
    f: F,
    marker: std::marker::PhantomData<fn() -> Input>,
}

trait System {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) -> ResultContainer;
}

type StoredSystem = Box<dyn System>;
struct Scheduler {
    systems: Vec<StoredSystem>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}

struct ResultContainer {
    result: Box<dyn Any>,
    type_id: TypeId,
}

trait IntoSystem<Input> {
    type System: System;
    fn into_system(self) -> Self::System;
}
impl<F: FnMut() -> Box<dyn Any>> IntoSystem<()> for F {
    type System = FunctionSystem<Self, ()>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
impl<F: FnMut(T1) -> Box<dyn Any>, T1: 'static> IntoSystem<(T1,)> for F {
    type System = FunctionSystem<Self, (T1,)>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
impl<F: FnMut(T1, T2) -> Box<dyn Any>, T1: 'static, T2: 'static> IntoSystem<(T1, T2)> for F {
    type System = FunctionSystem<Self, (T1, T2)>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}

impl<F: FnMut() -> Box<dyn Any>> System for FunctionSystem<F, ()> {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) -> ResultContainer {
        ResultContainer {
            result: (self.f)(),
            type_id: TypeId::of::<()>(),
        }
    }
}

impl<F: FnMut(I1) -> Box<dyn Any>, I1: 'static> System for FunctionSystem<F, (I1,)> {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) -> ResultContainer {
        let i1 = *resources
            .remove(&TypeId::of::<I1>())
            .unwrap()
            .downcast::<I1>()
            .unwrap();
        ResultContainer {
            result: (self.f)(i1),
            type_id: TypeId::of::<I1>(),
        }
    }
}

impl<F: FnMut(I1, I2) -> Box<dyn Any>, I1: 'static, I2: 'static> System
    for FunctionSystem<F, (I1, I2)>
{
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) -> ResultContainer {
        let i1 = *resources
            .remove(&TypeId::of::<I1>())
            .unwrap()
            .downcast::<I1>()
            .unwrap();
        let i2 = *resources
            .remove(&TypeId::of::<I1>())
            .unwrap()
            .downcast::<I2>()
            .unwrap();
        ResultContainer {
            result: (self.f)(i1, i2),
            type_id: TypeId::of::<I1>(),
        }
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
            // let res: &'static mut dyn Any = Box::leak(system.run(&mut self.resources));
            let result_container = system.run(&mut self.resources);
            self.resources
                .insert(result_container.type_id, result_container.result);
        }
    }

    pub fn add_resource<R: 'static>(&mut self, resource: R) -> &mut Self {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
        self
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S) -> &mut Self {
        self.systems.push(Box::new(system.into_system()));
        self
    }
}

fn get_i32() -> i32 {
    42
}

fn get_string() -> String {
    "Hello, World!".to_string()
}

fn start_system(i: i32, s: String) -> Box<dyn Any> {
    println!("i: {}, s: {}", i, s);
    Box::new(())
}

fn main() {
    Scheduler::new()
        .add_system(start_system)
        .add_resource(get_i32())
        .add_resource(get_string())
        .run();
    println!("list: {:?}", 2); // Prints "not a match"
}
