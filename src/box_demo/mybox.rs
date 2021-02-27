struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
// use std::ops::Drop;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBoxStr {
    data: String,
}

impl MyBoxStr {
    fn new(x: String) -> MyBoxStr {
        MyBoxStr { data: x }
    }
}

impl Drop for MyBoxStr {
    fn drop(&mut self) {
        println!("Dropping MyBoxStr data {}!", self.data);
    }
}

pub fn mybox_func() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    drop_order()
}

fn drop_order() {
    let _c = MyBoxStr::new(String::from("my stuff"));
    let _d = MyBoxStr::new(String::from("other stuff"));
    println!("MyBoxStr created.");
    let e = MyBoxStr::new(String::from("std::mem::drop"));
    drop(e);
}