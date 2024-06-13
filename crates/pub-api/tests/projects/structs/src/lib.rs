// ---- PubStruct1 ----
pub struct PubStruct1 {
    value: usize,
}

impl PubStruct1 {
    pub fn pub_struct1_function() {
        println!("pub struct1 function");
    }

    pub(crate) fn pub_crate_struct1_function() {
        println!("pub(crate) struct1 function");
    }

    fn private_struct1_function() {
        println!("private struct1 function");
    }
}

// ---- PubStruct2 ----
pub struct PubStruct2 {
    value: usize,
}

impl PubStruct2 {
    pub fn pub_struct2_function() {
        println!("pub struct2 function");
    }

    pub(crate) fn pub_crate_struct2_function() {
        println!("pub(crate) struct2 function");
    }

    fn private_struct2_function() {
        println!("private struct2 function");
    }
}

// ---- PubCrateStruct ----
pub(crate) struct PubCrateStruct {
    value: usize,
}

// ---- PrivateStruct ----
struct PrivateStruct {
    value: usize,
}
