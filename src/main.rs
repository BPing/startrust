mod generics_demo;
mod trait_demo;
mod macro_demo;
mod error_demo;
mod vector_demo;
mod box_demo;


use rand::Rng;


fn main() {
    let mut rng = rand::thread_rng();
    println!("Random f64: {}", rng.gen::<f64>());
    macro_demo::test();
    generics_demo::test();


    vector_demo::vector_func();
    vector_demo::vector_enum();
    vector_demo::vector_trait();

    box_demo::box_trait();
    box_demo::mybox_func();

    error_demo::test_result();
    error_demo::test_panic();
}

