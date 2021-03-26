pub mod mybox;
mod rc;

pub use mybox::mybox_func;


trait Fruit {
    fn get_name(&self) -> String;
}

#[derive(Clone)]
struct Apple;

impl Fruit for Apple {
    fn get_name(&self) -> String {
        String::from("Apple")
    }
}

#[derive(Clone)]
struct Pear;

impl Fruit for Pear {
    fn get_name(&self) -> String {
        String::from("Pear")
    }
}

pub fn box_trait() {
    let fruit_vec: Vec<Box<dyn Fruit>> = vec![Box::new(Apple), Box::new(Pear)];
    for f in &fruit_vec {
        println!("box fruit: {}", f.get_name())
    }
}