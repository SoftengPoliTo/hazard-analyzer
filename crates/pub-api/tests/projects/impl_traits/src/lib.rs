// ---- Traits ----
pub trait Trait1 {
    fn provided_function1() {
        println!("provided function1");
    }

    fn required_function1();
}

pub trait Trait2 {
    fn provided_function2() {
        println!("provided function2");
    }

    fn required_function2();
}

pub(crate) trait PubCrateTrait {
    fn pub_crate_provided_function() {
        println!("pub(crate) provided function");
    }

    fn pub_crate_required_function();
}

trait PrivateTrait {
    fn private_provided_function() {
        println!("private provided function");
    }

    fn private_required_function();
}

// ---- Struct ----
pub struct Struct {
    value: usize,
}

impl Trait1 for Struct {
    fn required_function1() {
        println!("impl Trait1 for Struct")
    }
}

impl Trait2 for Struct {
    fn required_function2() {
        println!("impl Trait2 for Struct")
    }
}

impl PubCrateTrait for Struct {
    fn pub_crate_required_function() {
        println!("impl PubCrateTrait for Struct")
    }
}

impl PrivateTrait for Struct {
    fn private_required_function() {
        println!("impl PrivateTrait for Struct")
    }
}
