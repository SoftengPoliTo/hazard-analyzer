#[macro_export]
macro_rules! pub_macro1 {
    () => {
        println!("pub macro1");
    };
}

#[macro_export]
macro_rules! pub_macro2 {
    () => {
        println!("pub macro2");
    };
}

macro_rules! pub_crate_macro {
    () => {
        println!("pub(crate) macro");
    };
}

macro_rules! private_macro {
    () => {
        println!("private macro");
    };
}
