use crate::errors::ShellError;
use crate::object::process::Process;
use crate::object::{ShellObject, Value};
use crate::prelude::*;
use crate::Command;
use derive_new::new;
use std::cell::RefCell;
use std::rc::Rc;
use sysinfo::SystemExt;

#[derive(new)]
pub struct PsBlueprint {
    system: Rc<RefCell<sysinfo::System>>,
}

impl crate::CommandBlueprint for PsBlueprint {
    fn create(
        &self,
        args: Vec<Value>,
        host: &dyn crate::Host,
        env: &mut crate::Environment,
    ) -> Result<Box<dyn Command>, ShellError> {
        Ok(Box::new(Ps::new(self.system.clone())))
    }
}

#[derive(new)]
pub struct Ps {
    system: Rc<RefCell<sysinfo::System>>,
}

impl crate::Command for Ps {
    fn run(&mut self, stream: VecDeque<Value>) -> Result<VecDeque<ReturnValue>, ShellError> {
        let mut system = self.system.borrow_mut();
        system.refresh_all();

        let list = system.get_process_list();

        let list = list
            .into_iter()
            .map(|(_, process)| {
                ReturnValue::Value(Value::Object(Box::new(Process::new(process.clone()))))
            })
            .take(5)
            .collect::<VecDeque<_>>();

        Ok(list)
    }
}