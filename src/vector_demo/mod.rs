pub fn vector_func() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    for i in &v {
        println!("{}", i)
    }
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i)
    }
}

enum MultiType {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vector_enum() {
    let row = vec![
        MultiType::Int(3),
        MultiType::Text(String::from("blue")),
        MultiType::Float(10.12),
    ];
    for i in &row {
        match i {
            MultiType::Int(i) => println!("The Int element is {}", i),
            MultiType::Float(i) => println!("The Float element is {}", i),
            MultiType::Text(i) => println!("The Text element is {}", i),
        }
    }
}


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

pub fn vector_trait() {
    let fruit_vec: Vec<&dyn Fruit> = vec![&Apple, &Pear];
    for f in &fruit_vec {
        println!("trait fruit: {}", f.get_name())
    }
}