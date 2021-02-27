macro_rules! hey {
    ($( $name:expr ),*) => {
    {
     $( print!("hey {}! ",$name); )*
     print!("\n")
    }
    }
}

pub fn test() {
    hey!("macro_rules","option");
}

