mod generics_demo;
mod trait_demo;
mod macro_demo;
mod error_demo;
mod vector_demo;
mod box_demo;
mod unsafe_demo;
mod transfer;
mod crust;
mod error_chain_demo;

fn main() {
    // crust::rust_cc_func();

    error_chain_demo::error_chain_func();

    unsafe_demo::unsafe_func();

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

