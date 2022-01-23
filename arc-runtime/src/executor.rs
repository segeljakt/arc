use crate::channels;
use crate::data::Data;
use crate::task::message::TaskMessage;

use std::cell::RefCell;
use std::lazy::OnceCell;

use derive_more::Constructor as New;
use kompact::prelude::*;
use std::sync::Arc;

pub struct Executor {
    actor: OnceCell<Arc<Component<ExecutorActor>>>,
    system: OnceCell<KompactSystem>,
    tasks: RefCell<Vec<ActorRef<TaskMessage>>>,
}

#[derive(Actor, ComponentDefinition)]
pub struct ExecutorActor {
    ctx: ComponentContext<Self>,
}

impl ExecutorActor {
    pub fn new() -> Self {
        ExecutorActor {
            ctx: ComponentContext::uninitialised(),
        }
    }
}

impl ComponentLifecycle for ExecutorActor {}

impl Executor {
    pub const fn new() -> Self {
        Self {
            actor: OnceCell::new(),
            system: OnceCell::new(),
            tasks: RefCell::new(Vec::new()),
        }
    }

    pub fn init(&self, system: KompactSystem) {
//         let (actor, path) = system.create_and_register(ExecutorActor::new);
//         self.actor.set(actor);
        assert!(self.system.set(system).is_ok())
    }

    pub fn create_task<C: ComponentDefinition + ActorRaw<Message = TaskMessage>>(
        &self,
        f: impl FnOnce() -> C,
    ) -> Arc<Component<C>> {
        let task = self.create(f);
        self.start(&task);
        self.tasks.borrow_mut().push(task.actor_ref());
        task
    }

    pub fn delete_task(&self) {
        let task = self.tasks.borrow_mut().pop().unwrap();
    }

    pub fn create_remote_task<C: ComponentDefinition>(
        &self,
        f: impl FnOnce() -> C,
    ) -> (Arc<Component<C>>, ActorPath) {
        let (task, fut) = self.create_and_register(f);
        let path = fut.wait().unwrap();
        self.start(&task);
        (task, path)
    }

    pub fn await_termination(&self) {
        unsafe {
            (*(self as *const Self as *mut Self))
                .system
                .take()
                .unwrap()
                .await_termination()
        }
    }
}

impl std::ops::Deref for Executor {
    type Target = KompactSystem;

    fn deref(&self) -> &Self::Target {
        self.system.get().unwrap()
    }
}

unsafe impl Sync for Executor {}
