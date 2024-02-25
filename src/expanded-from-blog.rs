#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::{
    any::{Any, TypeId},
    collections::HashMap, ops::Deref, sync::Arc,
};
pub struct TypeMap {
    bindings: HashMap<TypeId, Box<dyn Any>>,
}
#[automatically_derived]
impl ::core::default::Default for TypeMap {
    #[inline]
    fn default() -> TypeMap {
        TypeMap {
            bindings: ::core::default::Default::default(),
        }
    }
}
impl TypeMap {
    pub fn bind<T: Any>(&mut self, val: T) {
        self.bindings.insert(val.type_id(), Box::new(val));
    }
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.bindings.get(&TypeId::of::<T>()).and_then(|boxed| boxed.downcast_ref())
    }
    pub fn call<F, Args>(&self, callable: F)
    where
        F: Callable<Args>,
        Args: FromTypeMap,
    {
        callable.call(Args::from_type_map(self));
    }
}
pub trait FromTypeMap {
    fn from_type_map(type_map: &TypeMap) -> Self;
}
pub struct Data<T: ?Sized>(Arc<T>);
impl<T> Data<T> {
    pub fn new(val: T) -> Self {
        Data(Arc::new(val))
    }
}
impl<T: ?Sized> Data<T> {
    pub fn get(&self) -> &T {
        self.0.as_ref()
    }
}
impl<T: ?Sized> Clone for Data<T> {
    fn clone(&self) -> Data<T> {
        Data(self.0.clone())
    }
}
impl<T: ?Sized> Deref for Data<T> {
    type Target = Arc<T>;
    fn deref(&self) -> &Arc<T> {
        &self.0
    }
}
impl<T: ?Sized + 'static> FromTypeMap for Data<T> {
    fn from_type_map(type_map: &TypeMap) -> Self {
        type_map.get::<Self>().expect("type not found").clone()
    }
}
struct MyStruct {
    #[allow(dead_code)]
    val: i32,
}
#[automatically_derived]
impl ::core::marker::Copy for MyStruct {}
#[automatically_derived]
impl ::core::clone::Clone for MyStruct {
    #[inline]
    fn clone(&self) -> MyStruct {
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
impl FromTypeMap for MyStruct {
    fn from_type_map(type_map: &TypeMap) -> Self {
        *type_map.get::<Self>().expect("type not found")
    }
}
pub trait Callable<Args> {
    fn call(&self, args: Args);
}
impl<Func> Callable<()> for Func
where
    Func: Fn(),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(&self, (): ()) {
        (self)()
    }
}
impl<Func, A> Callable<(A,)> for Func
where
    Func: Fn(A),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(&self, (A,): (A,)) {
        (self)(A)
    }
}
impl<Func, A, B> Callable<(A, B)> for Func
where
    Func: Fn(A, B),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(&self, (A, B): (A, B)) {
        (self)(A, B)
    }
}
impl<Func, A, B, C> Callable<(A, B, C)> for Func
where
    Func: Fn(A, B, C),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(&self, (A, B, C): (A, B, C)) {
        (self)(A, B, C)
    }
}
impl<Func, A, B, C, D> Callable<(A, B, C, D)> for Func
where
    Func: Fn(A, B, C, D),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(&self, (A, B, C, D): (A, B, C, D)) {
        (self)(A, B, C, D)
    }
}
impl<Func, A, B, C, D, E> Callable<(A, B, C, D, E)> for Func
where
    Func: Fn(A, B, C, D, E),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(&self, (A, B, C, D, E): (A, B, C, D, E)) {
        (self)(A, B, C, D, E)
    }
}
impl<Func, A, B, C, D, E, F> Callable<(A, B, C, D, E, F)> for Func
where
    Func: Fn(A, B, C, D, E, F),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(&self, (A, B, C, D, E, F): (A, B, C, D, E, F)) {
        (self)(A, B, C, D, E, F)
    }
}
impl<Func, A, B, C, D, E, F, G> Callable<(A, B, C, D, E, F, G)> for Func
where
    Func: Fn(A, B, C, D, E, F, G),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(&self, (A, B, C, D, E, F, G): (A, B, C, D, E, F, G)) {
        (self)(A, B, C, D, E, F, G)
    }
}
impl<Func, A, B, C, D, E, F, G, H> Callable<(A, B, C, D, E, F, G, H)> for Func
where
    Func: Fn(A, B, C, D, E, F, G, H),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(&self, (A, B, C, D, E, F, G, H): (A, B, C, D, E, F, G, H)) {
        (self)(A, B, C, D, E, F, G, H)
    }
}
impl<Func, A, B, C, D, E, F, G, H, I> Callable<(A, B, C, D, E, F, G, H, I)> for Func
where
    Func: Fn(A, B, C, D, E, F, G, H, I),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(&self, (A, B, C, D, E, F, G, H, I): (A, B, C, D, E, F, G, H, I)) {
        (self)(A, B, C, D, E, F, G, H, I)
    }
}
impl<Func, A, B, C, D, E, F, G, H, I, J> Callable<(A, B, C, D, E, F, G, H, I, J)>
for Func
where
    Func: Fn(A, B, C, D, E, F, G, H, I, J),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(&self, (A, B, C, D, E, F, G, H, I, J): (A, B, C, D, E, F, G, H, I, J)) {
        (self)(A, B, C, D, E, F, G, H, I, J)
    }
}
impl<Func, A, B, C, D, E, F, G, H, I, J, K> Callable<(A, B, C, D, E, F, G, H, I, J, K)>
for Func
where
    Func: Fn(A, B, C, D, E, F, G, H, I, J, K),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(
        &self,
        (A, B, C, D, E, F, G, H, I, J, K): (A, B, C, D, E, F, G, H, I, J, K),
    ) {
        (self)(A, B, C, D, E, F, G, H, I, J, K)
    }
}
impl<
    Func,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
> Callable<(A, B, C, D, E, F, G, H, I, J, K, L)> for Func
where
    Func: Fn(A, B, C, D, E, F, G, H, I, J, K, L),
{
    #[inline]
    #[allow(non_snake_case)]
    fn call(
        &self,
        (A, B, C, D, E, F, G, H, I, J, K, L): (A, B, C, D, E, F, G, H, I, J, K, L),
    ) {
        (self)(A, B, C, D, E, F, G, H, I, J, K, L)
    }
}
impl<A: FromTypeMap> FromTypeMap for (A,) {
    #[inline]
    fn from_type_map(type_map: &TypeMap) -> Self {
        (A::from_type_map(type_map),)
    }
}
impl<A: FromTypeMap, B: FromTypeMap> FromTypeMap for (A, B) {
    #[inline]
    fn from_type_map(type_map: &TypeMap) -> Self {
        (A::from_type_map(type_map), B::from_type_map(type_map))
    }
}
impl<A: FromTypeMap, B: FromTypeMap, C: FromTypeMap> FromTypeMap for (A, B, C) {
    #[inline]
    fn from_type_map(type_map: &TypeMap) -> Self {
        (
            A::from_type_map(type_map),
            B::from_type_map(type_map),
            C::from_type_map(type_map),
        )
    }
}
impl<A: FromTypeMap, B: FromTypeMap, C: FromTypeMap, D: FromTypeMap> FromTypeMap
for (A, B, C, D) {
    #[inline]
    fn from_type_map(type_map: &TypeMap) -> Self {
        (
            A::from_type_map(type_map),
            B::from_type_map(type_map),
            C::from_type_map(type_map),
            D::from_type_map(type_map),
        )
    }
}
impl<
    A: FromTypeMap,
    B: FromTypeMap,
    C: FromTypeMap,
    D: FromTypeMap,
    E: FromTypeMap,
> FromTypeMap for (A, B, C, D, E) {
    #[inline]
    fn from_type_map(type_map: &TypeMap) -> Self {
        (
            A::from_type_map(type_map),
            B::from_type_map(type_map),
            C::from_type_map(type_map),
            D::from_type_map(type_map),
            E::from_type_map(type_map),
        )
    }
}
impl<
    A: FromTypeMap,
    B: FromTypeMap,
    C: FromTypeMap,
    D: FromTypeMap,
    E: FromTypeMap,
    F: FromTypeMap,
> FromTypeMap for (A, B, C, D, E, F) {
    #[inline]
    fn from_type_map(type_map: &TypeMap) -> Self {
        (
            A::from_type_map(type_map),
            B::from_type_map(type_map),
            C::from_type_map(type_map),
            D::from_type_map(type_map),
            E::from_type_map(type_map),
            F::from_type_map(type_map),
        )
    }
}
impl<
    A: FromTypeMap,
    B: FromTypeMap,
    C: FromTypeMap,
    D: FromTypeMap,
    E: FromTypeMap,
    F: FromTypeMap,
    G: FromTypeMap,
> FromTypeMap for (A, B, C, D, E, F, G) {
    #[inline]
    fn from_type_map(type_map: &TypeMap) -> Self {
        (
            A::from_type_map(type_map),
            B::from_type_map(type_map),
            C::from_type_map(type_map),
            D::from_type_map(type_map),
            E::from_type_map(type_map),
            F::from_type_map(type_map),
            G::from_type_map(type_map),
        )
    }
}
impl<
    A: FromTypeMap,
    B: FromTypeMap,
    C: FromTypeMap,
    D: FromTypeMap,
    E: FromTypeMap,
    F: FromTypeMap,
    G: FromTypeMap,
    H: FromTypeMap,
> FromTypeMap for (A, B, C, D, E, F, G, H) {
    #[inline]
    fn from_type_map(type_map: &TypeMap) -> Self {
        (
            A::from_type_map(type_map),
            B::from_type_map(type_map),
            C::from_type_map(type_map),
            D::from_type_map(type_map),
            E::from_type_map(type_map),
            F::from_type_map(type_map),
            G::from_type_map(type_map),
            H::from_type_map(type_map),
        )
    }
}
impl<
    A: FromTypeMap,
    B: FromTypeMap,
    C: FromTypeMap,
    D: FromTypeMap,
    E: FromTypeMap,
    F: FromTypeMap,
    G: FromTypeMap,
    H: FromTypeMap,
    I: FromTypeMap,
> FromTypeMap for (A, B, C, D, E, F, G, H, I) {
    #[inline]
    fn from_type_map(type_map: &TypeMap) -> Self {
        (
            A::from_type_map(type_map),
            B::from_type_map(type_map),
            C::from_type_map(type_map),
            D::from_type_map(type_map),
            E::from_type_map(type_map),
            F::from_type_map(type_map),
            G::from_type_map(type_map),
            H::from_type_map(type_map),
            I::from_type_map(type_map),
        )
    }
}
impl<
    A: FromTypeMap,
    B: FromTypeMap,
    C: FromTypeMap,
    D: FromTypeMap,
    E: FromTypeMap,
    F: FromTypeMap,
    G: FromTypeMap,
    H: FromTypeMap,
    I: FromTypeMap,
    J: FromTypeMap,
> FromTypeMap for (A, B, C, D, E, F, G, H, I, J) {
    #[inline]
    fn from_type_map(type_map: &TypeMap) -> Self {
        (
            A::from_type_map(type_map),
            B::from_type_map(type_map),
            C::from_type_map(type_map),
            D::from_type_map(type_map),
            E::from_type_map(type_map),
            F::from_type_map(type_map),
            G::from_type_map(type_map),
            H::from_type_map(type_map),
            I::from_type_map(type_map),
            J::from_type_map(type_map),
        )
    }
}
impl<
    A: FromTypeMap,
    B: FromTypeMap,
    C: FromTypeMap,
    D: FromTypeMap,
    E: FromTypeMap,
    F: FromTypeMap,
    G: FromTypeMap,
    H: FromTypeMap,
    I: FromTypeMap,
    J: FromTypeMap,
    K: FromTypeMap,
> FromTypeMap for (A, B, C, D, E, F, G, H, I, J, K) {
    #[inline]
    fn from_type_map(type_map: &TypeMap) -> Self {
        (
            A::from_type_map(type_map),
            B::from_type_map(type_map),
            C::from_type_map(type_map),
            D::from_type_map(type_map),
            E::from_type_map(type_map),
            F::from_type_map(type_map),
            G::from_type_map(type_map),
            H::from_type_map(type_map),
            I::from_type_map(type_map),
            J::from_type_map(type_map),
            K::from_type_map(type_map),
        )
    }
}
impl<
    A: FromTypeMap,
    B: FromTypeMap,
    C: FromTypeMap,
    D: FromTypeMap,
    E: FromTypeMap,
    F: FromTypeMap,
    G: FromTypeMap,
    H: FromTypeMap,
    I: FromTypeMap,
    J: FromTypeMap,
    K: FromTypeMap,
    L: FromTypeMap,
> FromTypeMap for (A, B, C, D, E, F, G, H, I, J, K, L) {
    #[inline]
    fn from_type_map(type_map: &TypeMap) -> Self {
        (
            A::from_type_map(type_map),
            B::from_type_map(type_map),
            C::from_type_map(type_map),
            D::from_type_map(type_map),
            E::from_type_map(type_map),
            F::from_type_map(type_map),
            G::from_type_map(type_map),
            H::from_type_map(type_map),
            I::from_type_map(type_map),
            J::from_type_map(type_map),
            K::from_type_map(type_map),
            L::from_type_map(type_map),
        )
    }
}
