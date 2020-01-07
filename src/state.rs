/// Base trait for states that defines a life cycle.
pub trait State {
    fn on_pause(&self) {}
    fn on_resume(&self) {}

    fn on_update(&mut self, delta: f32);
    fn on_render(&self);
}

/// Trait for states that can only be created with some data provided.
pub trait WithData {
    type Data;
    fn new(data: Self::Data) -> Self;
}

/// Trait for states that can be created without any data.
pub trait WithoutData {
    fn new() -> Self;
}


/// A structure to manage states using a state stack.
pub struct StateManager<'a> {
   stack: Vec<Box<dyn State + 'a>>
}

impl<'a> StateManager<'a> {
    /// Creates an empty manager.
    pub fn new() -> StateManager<'a> {
        StateManager { stack: Vec::new() }
    }

    /// Renders a whole stack.
    pub fn render(&self) {
        for state in &self.stack {
            state.on_render();
        }
    }

    /// Updates a whole stack.
    pub fn update(&mut self, delta: f32) {
        for state in &mut self.stack {
            state.on_update(delta);
        }
    }

    /// Push a new state with required data on top of the stack and 
    /// switch to it.
    pub fn forward_with<S, D>(&mut self, data: D)
        where S: State + WithData<Data=D> + 'a
    {
        let state = S::new(data);
        self.stack.push(Box::new(state));
    }

    /// Push a new state on top of the stack and switch to it.
    pub fn forward<S>(&mut self) 
        where S: State + WithoutData + 'a
    {
        let state = S::new();
        self.stack.push(Box::new(state));
    }
}
