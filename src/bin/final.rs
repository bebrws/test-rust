#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
trait SystemParam {
    type Item<'new>;
    fn retrieve<'r>(resources: &'r HashMap<TypeId, RefCell<Box<dyn Any>>>) -> Self::Item<'r>;
}
impl<'res, T: 'static> SystemParam for Res<'res, T> {
    type Item<'new> = Res<'new, T>;
    fn retrieve<'r>(resources: &'r HashMap<TypeId, RefCell<Box<dyn Any>>>) -> Self::Item<'r> {
        Res {
            value: resources.get(&TypeId::of::<T>()).unwrap().borrow(),
            _marker: PhantomData,
        }
    }
}
impl<'res, T: 'static> SystemParam for ResMut<'res, T> {
    type Item<'new> = ResMut<'new, T>;
    fn retrieve<'r>(resources: &'r HashMap<TypeId, RefCell<Box<dyn Any>>>) -> Self::Item<'r> {
        ResMut {
            value: resources.get(&TypeId::of::<T>()).unwrap().borrow_mut(),
            _marker: PhantomData,
        }
    }
}
struct Res<'a, T: 'static> {
    value: Ref<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a T>,
}
impl<T: 'static> Deref for Res<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}
struct ResMut<'a, T: 'static> {
    value: RefMut<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a mut T>,
}
impl<T: 'static> Deref for ResMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}
impl<T: 'static> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.downcast_mut().unwrap()
    }
}
struct FunctionSystem<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}
trait System {
    fn run(&mut self, resources: &mut HashMap<TypeId, RefCell<Box<dyn Any>>>);
}
#[allow(non_snake_case)]
#[allow(unused)]
impl<F> System for FunctionSystem<(), F>
where
    for<'a, 'b> &'a mut F: FnMut() + FnMut(),
{
    fn run(&mut self, resources: &mut HashMap<TypeId, RefCell<Box<dyn Any>>>) {
        fn call_inner(mut f: impl FnMut()) {
            f()
        }
        call_inner(&mut self.f)
    }
}
#[allow(non_snake_case)]
#[allow(unused)]
impl<F, T1: SystemParam> System for FunctionSystem<(T1,), F>
where
    for<'a, 'b> &'a mut F: FnMut(T1) + FnMut(<T1 as SystemParam>::Item<'b>),
{
    fn run(&mut self, resources: &mut HashMap<TypeId, RefCell<Box<dyn Any>>>) {
        fn call_inner<T1>(mut f: impl FnMut(T1), T1: T1) {
            f(T1)
        }
        let T1 = T1::retrieve(resources);
        call_inner(&mut self.f, T1)
    }
}
#[allow(non_snake_case)]
#[allow(unused)]
impl<F, T1: SystemParam, T2: SystemParam> System for FunctionSystem<(T1, T2), F>
where
    for<'a, 'b> &'a mut F:
        FnMut(T1, T2) + FnMut(<T1 as SystemParam>::Item<'b>, <T2 as SystemParam>::Item<'b>),
{
    fn run(&mut self, resources: &mut HashMap<TypeId, RefCell<Box<dyn Any>>>) {
        fn call_inner<T1, T2>(mut f: impl FnMut(T1, T2), T1: T1, T2: T2) {
            f(T1, T2)
        }
        let T1 = T1::retrieve(resources);
        let T2 = T2::retrieve(resources);
        call_inner(&mut self.f, T1, T2)
    }
}
#[allow(non_snake_case)]
#[allow(unused)]
impl<F, T1: SystemParam, T2: SystemParam, T3: SystemParam> System
    for FunctionSystem<(T1, T2, T3), F>
where
    for<'a, 'b> &'a mut F: FnMut(T1, T2, T3)
        + FnMut(
            <T1 as SystemParam>::Item<'b>,
            <T2 as SystemParam>::Item<'b>,
            <T3 as SystemParam>::Item<'b>,
        ),
{
    fn run(&mut self, resources: &mut HashMap<TypeId, RefCell<Box<dyn Any>>>) {
        fn call_inner<T1, T2, T3>(mut f: impl FnMut(T1, T2, T3), T1: T1, T2: T2, T3: T3) {
            f(T1, T2, T3)
        }
        let T1 = T1::retrieve(resources);
        let T2 = T2::retrieve(resources);
        let T3 = T3::retrieve(resources);
        call_inner(&mut self.f, T1, T2, T3)
    }
}
#[allow(non_snake_case)]
#[allow(unused)]
impl<F, T1: SystemParam, T2: SystemParam, T3: SystemParam, T4: SystemParam> System
    for FunctionSystem<(T1, T2, T3, T4), F>
where
    for<'a, 'b> &'a mut F: FnMut(T1, T2, T3, T4)
        + FnMut(
            <T1 as SystemParam>::Item<'b>,
            <T2 as SystemParam>::Item<'b>,
            <T3 as SystemParam>::Item<'b>,
            <T4 as SystemParam>::Item<'b>,
        ),
{
    fn run(&mut self, resources: &mut HashMap<TypeId, RefCell<Box<dyn Any>>>) {
        fn call_inner<T1, T2, T3, T4>(
            mut f: impl FnMut(T1, T2, T3, T4),
            T1: T1,
            T2: T2,
            T3: T3,
            T4: T4,
        ) {
            f(T1, T2, T3, T4)
        }
        let T1 = T1::retrieve(resources);
        let T2 = T2::retrieve(resources);
        let T3 = T3::retrieve(resources);
        let T4 = T4::retrieve(resources);
        call_inner(&mut self.f, T1, T2, T3, T4)
    }
}
trait IntoSystem<Input> {
    type System: System;
    fn into_system(self) -> Self::System;
}
impl<F> IntoSystem<()> for F
where
    for<'a, 'b> &'a mut F: FnMut() + FnMut(),
{
    type System = FunctionSystem<(), Self>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
impl<F, T1: SystemParam> IntoSystem<(T1,)> for F
where
    for<'a, 'b> &'a mut F: FnMut(T1) + FnMut(<T1 as SystemParam>::Item<'b>),
{
    type System = FunctionSystem<(T1,), Self>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
impl<F, T1: SystemParam, T2: SystemParam> IntoSystem<(T1, T2)> for F
where
    for<'a, 'b> &'a mut F:
        FnMut(T1, T2) + FnMut(<T1 as SystemParam>::Item<'b>, <T2 as SystemParam>::Item<'b>),
{
    type System = FunctionSystem<(T1, T2), Self>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
impl<F, T1: SystemParam, T2: SystemParam, T3: SystemParam> IntoSystem<(T1, T2, T3)> for F
where
    for<'a, 'b> &'a mut F: FnMut(T1, T2, T3)
        + FnMut(
            <T1 as SystemParam>::Item<'b>,
            <T2 as SystemParam>::Item<'b>,
            <T3 as SystemParam>::Item<'b>,
        ),
{
    type System = FunctionSystem<(T1, T2, T3), Self>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
impl<F, T1: SystemParam, T2: SystemParam, T3: SystemParam, T4: SystemParam>
    IntoSystem<(T1, T2, T3, T4)> for F
where
    for<'a, 'b> &'a mut F: FnMut(T1, T2, T3, T4)
        + FnMut(
            <T1 as SystemParam>::Item<'b>,
            <T2 as SystemParam>::Item<'b>,
            <T3 as SystemParam>::Item<'b>,
            <T4 as SystemParam>::Item<'b>,
        ),
{
    type System = FunctionSystem<(T1, T2, T3, T4), Self>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
type StoredSystem = Box<dyn System>;
struct Scheduler {
    systems: Vec<StoredSystem>,
    resources: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}
#[automatically_derived]
impl ::core::default::Default for Scheduler {
    #[inline]
    fn default() -> Scheduler {
        Scheduler {
            systems: ::core::default::Default::default(),
            resources: ::core::default::Default::default(),
        }
    }
}
impl Scheduler {
    pub fn run(&mut self) {
        for system in self.systems.iter_mut() {
            system.run(&mut self.resources);
        }
    }
    pub fn add_system<I, S: System + 'static>(&mut self, system: impl IntoSystem<I, System = S>) {
        self.systems.push(Box::new(system.into_system()));
    }
    pub fn add_resource<R: 'static>(&mut self, res: R) {
        self.resources
            .insert(TypeId::of::<R>(), RefCell::new(Box::new(res)));
    }
}
fn main() {
    let mut scheduler = Scheduler::default();
    scheduler.add_system(foo);
    scheduler.add_system(bar);
    scheduler.add_resource(12i32);
    scheduler.add_resource("Hello, world!");
    scheduler.run();
}
fn foo(mut int: ResMut<i32>) {
    *int += 1;
}
fn bar(statement: Res<&'static str>, num: Res<i32>) {
    {
        println!("{0} My lucky number is: {1}\n", *statement, *num);
    };
}
