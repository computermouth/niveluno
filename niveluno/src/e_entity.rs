// would rather match on sum type, but calling
// other_e::func seems hairy. maybe if it took a copy
// of other_e, modified, bubbled up to a buffer to replace?

pub trait EntityInstance {
    fn update(&mut self);
    //     fn update_physics(&mut self);
    //     fn collides(&mut self);
    //     fn did_collide(&mut self);
    //     fn did_collide_with_entity(&mut self);
    fn draw_model(&mut self);
    //     fn spawn_particles(&mut self);
    //     fn recv_damage(&mut self);
    //     fn play_sound(&mut self);
    //     fn kill(&mut self);
    //     fn pickup(&mut self);
    //     fn set_state(&mut self);
    //     fn spawn_projectile(&mut self);
    //     fn attack(&mut self);
}
