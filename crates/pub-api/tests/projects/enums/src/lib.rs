// ---- PubEnum1 ----
pub enum PubEnum1 {
    One,
    Two,
}

impl PubEnum1 {
    pub fn pub_enum1_function() {
        println!("pub enum1 function");
    }

    pub(crate) fn pub_crate_enum1_function() {
        println!("pub(crate) enum1 function");
    }

    fn private_enum1_function() {
        println!("private enum1 function");
    }
}

// ---- PubEnum1 ----
pub enum PubEnum2 {
    One,
    Two,
}

impl PubEnum2 {
    pub fn pub_enum2_function() {
        println!("pub enum2 function");
    }

    pub(crate) fn pub_crate_enum2_function() {
        println!("pub(crate) enum2 function");
    }

    fn private_enum2_function() {
        println!("private enum2 function");
    }
}

// ---- PubCrateEnum ----
pub(crate) enum PubCrateEnum {
    One,
    Two,
}

// ---- PrivateEnum ----
enum PrivateEnum {
    One,
    Two,
}
