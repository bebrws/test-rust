fn get_adder(a: i32) -> Box<dyn Fn(&i32) -> i32> {
    return Box::new(move |x| x + a);
}

fn create_adder(adder: fn(i32) -> Box<dyn Fn(&i32) -> i32>, x: i32) -> Box<dyn Fn(&i32) -> i32> {
    return get_adder(3);
}

fn main() {
    let mut val = 3;
    let mul: fn(&i32) -> i32 = |x| x * 2;
    let list = vec![1, 3, 5, 7, 9];
    let list_out: Vec<i32> = list.iter().map(create_adder(get_adder, 3)).collect();
    val = 4;
    println!("list: {:?}", list_out); // Prints "not a match"
}
