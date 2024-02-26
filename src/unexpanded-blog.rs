# From:
# https://nickbryan.co.uk/software/using-a-type-map-for-dependency-injection-in-rust/

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::Deref,
    sync::Arc,
};

// TypeMap is our dependency "container" and provides us with ways to store and retrieve some value
// bound to its type.
//
// The `call` method allows dependencies to be automatically injected into a function with an argument
// length of up to twelve.
#[derive(Default)]
pub struct TypeMap {
    bindings: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeMap {
    // bind stores the given value against its type within the container.
    pub fn bind<T: Any>(&mut self, val: T) {
        self.bindings.insert(val.type_id(), Box::new(val));
    }

    // get retrieves a reference to the value stored against the given type.
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.bindings
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }

    // call calls the given callable with its arguments resolved from the values bound to their
    // types within the container.
    pub fn call<F, Args>(&self, callable: F)
    where
        F: Callable<Args>,
        Args: FromTypeMap,
    {
        callable.call(Args::from_type_map(self));
    }
}

// FromTypeMap gives us a way to build some user defined type from the container.
pub trait FromTypeMap {
    fn from_type_map(type_map: &TypeMap) -> Self;
}

// Data is a container for a `T` that we can implement `FromTypeMap` on to allow a way for
// the users of our type-map to have any type resolved automatically into the callable.
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

// This allows the Data<T> to be built from the container.
impl<T: ?Sized + 'static> FromTypeMap for Data<T> {
    fn from_type_map(type_map: &TypeMap) -> Self {
        type_map.get::<Self>().expect("type not found").clone()
    }
}

// An example of how we can implement FromTypeMap for a user defined type.
#[derive(Copy, Clone)]
struct MyStruct {
    #[allow(dead_code)]
    val: i32,
}

impl FromTypeMap for MyStruct {
    fn from_type_map(type_map: &TypeMap) -> Self {
        *type_map.get::<Self>().expect("type not found")
    }
}

pub trait Callable<Args> {
    fn call(&self, args: Args);
}

// Here we implement `Callable` for tuples up to a length of twelve. Fn(A, B, C) for a tuple of three etc.
macro_rules! callable_tuple ({ $($param:ident)* } => {
    impl<Func, $($param,)*> Callable<($($param,)*)> for Func
    where
        Func: Fn($($param),*),
    {
        #[inline]
        #[allow(non_snake_case)]
        fn call(&self, ($($param,)*): ($($param,)*)) {
            (self)($($param,)*)
        }
    }
});

callable_tuple! {}
callable_tuple! { A }
callable_tuple! { A B }
callable_tuple! { A B C }
callable_tuple! { A B C D }
callable_tuple! { A B C D E }
callable_tuple! { A B C D E F }
callable_tuple! { A B C D E F G }
callable_tuple! { A B C D E F G H }
callable_tuple! { A B C D E F G H I }
callable_tuple! { A B C D E F G H I J }
callable_tuple! { A B C D E F G H I J K }
callable_tuple! { A B C D E F G H I J K L }

// Here we implement `FromTypeMap` for tuples up to a length of twelve. When `from_type_map` is
// called on the tuple it will return a new tuple with all of its arguments resolved from the container.
macro_rules! tuple_from_tm {
        ( $($T: ident )+ ) => {
            impl<$($T: FromTypeMap),+> FromTypeMap for ($($T,)+)
            {
                #[inline]
                fn from_type_map(type_map: &TypeMap) -> Self {
                    ($($T::from_type_map(type_map),)+)
                }
            }
        };
    }

tuple_from_tm! { A }
tuple_from_tm! { A B }
tuple_from_tm! { A B C }
tuple_from_tm! { A B C D }
tuple_from_tm! { A B C D E }
tuple_from_tm! { A B C D E F }
tuple_from_tm! { A B C D E F G }
tuple_from_tm! { A B C D E F G H }
tuple_from_tm! { A B C D E F G H I }
tuple_from_tm! { A B C D E F G H I J }
tuple_from_tm! { A B C D E F G H I J K }
tuple_from_tm! { A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_type_can_be_bound_and_resolved() {
        let mut container = TypeMap::default();
        container.bind::<i32>(42);
        assert_eq!(container.get::<i32>(), Some(&42));
    }

    #[test]
    fn a_type_can_be_bound_and_resolved_through_inference() {
        let mut container = TypeMap::default();
        container.bind(42);
        assert_eq!(container.get(), Some(&42));
    }

    #[test]
    fn injects_dependency_based_on_argument_type() {
        let mut container = TypeMap::default();
        container.bind(Data::new(42));
        container.call(|data: Data<i32>| {
            assert_eq!(data.get(), &42);
        });
    }

    #[test]
    fn injects_multiple_dependencies_based_on_argument_type() {
        let mut container = TypeMap::default();

        struct Point {
            x: u8,
            y: u8,
        }

        container.bind(Data::new(42));
        container.bind(Data::new(String::from("this is a test")));
        container.bind(Data::new(Point { x: 3, y: 5 }));
        container.bind(MyStruct { val: 7 });

        container.call(
            |string: Data<String>,
             number: Data<i32>,
             p1: Data<Point>,
             p2: Data<Point>,
             my_struct: MyStruct| {
                assert_eq!(string.as_str(), "this is a test");
                assert_eq!(number.get(), &42);
                assert_eq!(p1.x, 3);
                assert_eq!(p1.y, 5);
                assert_eq!(p2.x, 3);
                assert_eq!(p2.y, 5);
                assert_eq!(my_struct.val, 7);
            },
        );
    }

    #[test]
    fn the_values_methods_can_be_accessed_through_deref() {
        let mut container = TypeMap::default();
        container.bind(Data::new(String::from("test test 123")));
        container.call(|data: Data<String>| {
            assert_eq!(data.as_str(), "test test 123");
        });
    }
}