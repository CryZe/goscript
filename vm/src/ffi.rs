use super::value::GosValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type FfiResult<T> = std::result::Result<T, String>;
pub type Ctor = dyn Fn(Vec<GosValue>) -> FfiResult<Rc<RefCell<dyn Ffi>>>;

pub trait Ffi {
    fn call(&self, func_name: &str, params: Vec<GosValue>) -> Vec<GosValue>;
}

impl std::fmt::Debug for dyn Ffi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "ffi obj")
    }
}

pub struct FfiFactory {
    registry: HashMap<&'static str, Box<Ctor>>,
}

impl FfiFactory {
    pub fn new() -> FfiFactory {
        let mut f = FfiFactory {
            registry: HashMap::new(),
        };
        f.register("test", Box::new(TestFfi::new));
        f
    }

    pub fn register(&mut self, name: &'static str, ctor: Box<Ctor>) {
        self.registry.insert(name, ctor);
    }

    pub fn create_by_name(
        &self,
        name: &str,
        params: Vec<GosValue>,
    ) -> FfiResult<Rc<RefCell<dyn Ffi>>> {
        match self.registry.get(name) {
            Some(ctor) => (*ctor)(params),
            None => Err(format!("FFI named {} not found", name)),
        }
    }
}

pub struct TestFfi {}

impl TestFfi {
    pub fn new(_: Vec<GosValue>) -> FfiResult<Rc<RefCell<dyn Ffi>>> {
        Ok(Rc::new(RefCell::new(TestFfi {})))
    }
}

impl Ffi for TestFfi {
    fn call(&self, func_name: &str, params: Vec<GosValue>) -> Vec<GosValue> {
        vec![]
    }
}
