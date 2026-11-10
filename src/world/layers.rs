use avian2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Default)]
pub enum WorldLayer {
    #[default]
    Floor,
    Background,
    Enemies,
    Lander,
    Package,
    Target,
    Props,
}
