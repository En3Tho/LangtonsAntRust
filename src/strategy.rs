pub trait Strategy {
    fn go(&self, prevState: i32) -> (i32, Vec<bool>);
}

pub struct RlAntStrategy {
}

impl Strategy for RlAntStrategy {
    fn go(&self, prevState: i32) -> (i32, Vec<bool>) {
        let mut steps: Vec<bool> = Vec::new();
        let new_state;
        match prevState {
            0 => {
                steps.push(true);
                new_state = 1;
            }
            1 => {
                steps.push(false);
                new_state = 0;
            }
            _ => panic!("shit happened"),
        }
        (new_state, steps)
    }
}
