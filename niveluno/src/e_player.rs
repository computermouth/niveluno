use crate::e_entity::EntityInstance;
use crate::level::Entity;

pub struct Player {}

impl EntityInstance for Player {
    fn update(&mut self) {}
}

impl Player {
    pub fn new(entt: &Entity) -> Self {
        Self {}
    }
}
