pub trait PubTrait1 {
    fn pub_provided_function1() {
        println!("pub provided function1");
    }

    fn pub_required_function1();
}

pub trait PubTrait2 {
    fn pub_provided_function2() {
        println!("pub provided function2");
    }

    fn pub_required_function2();
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
