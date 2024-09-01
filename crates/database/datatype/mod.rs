//! TODO.
//!

use std::collections::HashMap;

pub struct Workflow {}

// TODO; Secret/Auth/Integration/Environment

pub struct Registry {
    pub services: HashMap<String, ()>,
    pub nodes: HashMap<String, ()>,
    pub edges: HashMap<String, ()>,
}

pub struct Entity {
    pub name: String,
    pub icon: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

pub struct Input {}

pub struct Output {}

pub enum Params {
    Bool(bool),
    Integer(i32),
    Float(f32),
    String(String),
}

pub enum ParamMeta {
    Bool(Option<Limit<bool>>),
}

pub enum ParamType {
    Bool,
    Integer,
    Float,
    String,
}

pub enum Limit<T> {
    MinOnly(T),
    MaxOnly(T),
    MinMax(T, T),
    Enum(Vec<T>),
}
