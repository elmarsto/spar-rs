use serde::{Serialize, Deserialize};

pub trait Ast {
  fn translate(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub struct Document {
  // TODO
}
#[derive(Serialize, Deserialize)]
pub struct Tastie {
  pub document: Document
}

pub impl Ast for Tastie {

}

