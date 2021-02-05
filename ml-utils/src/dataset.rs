#![allow(unused_imports)]
use std::io::prelude::*;
use std::vec::Vec;
#[derive(Debug, Deserialize)]
pub struct Flower {
    sepal_length: f32, // everything needs to be f32, other types wont do in rusty machine
    sepal_width: f32,
    petal_length: f32,
    petal_width: f32,
    species: String,
}
//Id,sepal_length,sepal_width,petal_length,petal_width,species
impl Flower {
    pub fn into_feature_vector(&self) -> Vec<f32> {
        vec![self.sepal_length, self.sepal_width, self.petal_length, self.petal_width]
    }

    pub fn into_labels(&self) -> f32 {
        match self.species.as_str() {
            "setosa" => 0.,
            "versicolor" => 1.,
            "virginica" => 2.,
            l => panic!("Not able to parse the target. Some other target got passed. {:?}", l),
        }
    }

    pub fn into_int_labels(&self) -> u64 {
        match self.species.as_str() {
            "setosa" => 0,
            "versicolor" => 1,
            "virginica" => 2,
            l => panic!("Not able to parse the target. Some other target got passed. {:?}", l),
        }
    }
}
