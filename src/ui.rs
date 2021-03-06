use crate::{
    bloom::Bloom,
    context::{ContextState, UpdateCtx},
    id::ChildCounter,
    key::Caller,
    object::{AnyRenderObject, Properties, RenderObject},
    tree::{Child, ChildState, Children, State},
};
use core::panic;
use std::any::Any;

pub struct Ui<'a, 'b> {
    tree: &'a mut Children,
    state: &'a mut ContextState<'b>,
    child_counter: &'a mut ChildCounter,
    state_index: usize,
    render_index: usize,
}

impl<'a, 'b> Ui<'a, 'b> {
    pub(crate) fn new(
        tree: &'a mut Children,
        state: &'a mut ContextState<'b>,
        child_counter: &'a mut ChildCounter,
    ) -> Self {
        Ui {
            tree,
            state,
            child_counter,
            state_index: 0,
            render_index: 0,
        }
    }

    pub fn state_node<T, I, N>(&mut self, caller: Caller, init: I, content: N)
    where
        T: Any,
        I: FnOnce() -> T,
        N: FnOnce(&mut Ui, &mut T),
    {
        let index = self.find_state_node(caller);
        if index.is_none() {
            self.insert_state_node(caller, Box::new(init()));
        }
        let index = index.unwrap_or(self.state_index);

        for node in &mut self.tree.states[self.state_index..index] {
            node.dead = true;
        }

        let node_prt = &mut self.tree.states[index] as *mut State;
        let node = unsafe { &mut *node_prt };
        self.state_index = index + 1;

        if let Some(state) = node.state.downcast_mut::<T>() {
            content(self, state);
        } else {
            // TODO: Handle wrong type of state
            panic!(
                "Wrong type of state. Expected {}",
                std::any::type_name::<T>()
            );
        }
    }

    pub fn render_object<P, R, N>(&mut self, caller: Caller, props: P, content: N) -> R::Action
    where
        P: Properties<Object = R>,
        R: RenderObject<P> + Any,
        N: FnOnce(&mut Ui),
    {
        let mut props = Some(props);
        let index = match self.find_render_object(caller) {
            Some(index) => index,
            None => {
                let object = R::create(props.take().unwrap());
                self.insert_render_object(caller, Box::new(object));
                self.find_render_object(caller).unwrap()
            }
        };
        for node in &mut self.tree.renders[self.render_index..index] {
            node.dead = true;
        }
        let node = &mut self.tree.renders[index];
        self.render_index = index + 1;

        let mut action = R::Action::default();
        if let Some(props) = props {
            if let Some(object) = node.object.as_any().downcast_mut::<R>() {
                let mut ctx = UpdateCtx {
                    state: self.state,
                    child_state: &mut node.state,
                };
                action = object.update(&mut ctx, props);
                node.state.request_update = false;
            } else {
                // TODO: Think of something smart
                panic!("Wrong node type. Expected {}", std::any::type_name::<R>())
            }
        }

        let mut object_cx = Ui::new(&mut node.children, self.state, self.child_counter);
        content(&mut object_cx);

        object_cx.tree.states.truncate(object_cx.state_index);
        object_cx.tree.states.retain(|s| !s.dead);
        object_cx.tree.renders.truncate(object_cx.render_index);
        object_cx.tree.renders.retain(|c| !c.dead);

        if true {
            // TODO: Only rebuild when children change.
            // Rebuild the bloom filter.
            node.state.children = node.children.renders.iter().map(|c| &c.state).fold(
                Bloom::new(),
                |mut bloom, child_state| {
                    bloom.add(&child_state.id);
                    bloom.union(child_state.children)
                },
            );
        }

        action
    }
}

impl Ui<'_, '_> {
    fn find_state_node(&mut self, caller: Caller) -> Option<usize> {
        let mut ix = self.state_index;
        for node in &mut self.tree.states[ix..] {
            if node.key == caller {
                return Some(ix);
            }
            ix += 1;
        }
        None
    }

    fn insert_state_node(&mut self, caller: Caller, state: Box<dyn Any>) {
        let key = caller;
        let dead = false;
        self.tree
            .states
            .insert(self.state_index, State { key, state, dead });
    }

    fn find_render_object(&mut self, caller: Caller) -> Option<usize> {
        let mut ix = self.render_index;
        for node in &mut self.tree.renders[ix..] {
            if node.key == caller {
                return Some(ix);
            }
            ix += 1;
        }
        None
    }

    fn insert_render_object(&mut self, caller: Caller, object: Box<dyn AnyRenderObject>) {
        self.tree.renders.insert(
            self.render_index,
            Child {
                key: caller,
                object,
                children: Children::new(),
                state: ChildState::new(self.child_counter.generate_id(), None),
                dead: false,
            },
        );
    }
}
