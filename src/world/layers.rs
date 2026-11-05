use avian2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Default)]
pub enum WorldLayer {
    #[default]
    Floor,
    Enemies,
    Lander,
    Props,
}
