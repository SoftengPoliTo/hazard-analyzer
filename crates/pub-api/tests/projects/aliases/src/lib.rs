pub mod first;
pub mod second;

use crate::{
    first::{a::S, b::S as S2},
    second::{a::S as S3, b::S as S4},
};

impl S {
    pub fn new_first_a_s_function() {
        println!("new first::a::S::function()");
    }
}

impl S2 {
    pub fn new_first_b_s_function() {
        println!("new first::b::S::function()");
    }
}

impl S3 {
    pub fn new_second_a_s_function() {
        println!("new second::a::S::function()");
    }
}

impl S4 {
    pub fn new_second_b_s_function() {
        println!("new second::b::S::function()");
    }
}
