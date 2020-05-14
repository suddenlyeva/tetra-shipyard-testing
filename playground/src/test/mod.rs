use crate::game::*;
mod systems;
use systems::*;
use shipyard::*;
use Input::*;
use InputAction::*;

pub fn test_state() -> GameState {
    let world = World::new();
    let mut controls = Controls::new();

    // This creates a "Hello" workload and binds it to the enter key
    controls.insert(Pressed(KeyEnter), "Hello");
    world.add_workload("Hello")
        .with_system(system!(hello_system))
        .build();

    // Right now the monostate always requires a workload to run so I make an empty one
    // We can consider wrapping it in an option but empty states are like not the most common use case so maybe it ok for now
    world.add_workload("Test").build();

    GameState::new("Test", world, controls)
}