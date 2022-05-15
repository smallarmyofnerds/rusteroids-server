use crate::inputs::Inputs;

pub struct Player {
    inputs: Inputs,
}

impl Player {
    pub fn new() -> Self {
        Self {
            inputs: Inputs {
                left: false,
                right: false,
                up: false,
                fire: false,
            },
        }
    }
    pub fn get_inputs(&self) -> Inputs {
        self.inputs.clone()
    }
}
