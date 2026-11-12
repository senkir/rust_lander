use bevy::prelude::{Entity, Message, MessageReader};

#[derive(Message)]
pub struct DeathTrigger {
    pub entity: Entity,
}
pub fn spawn_explosion(
    mut reader: MessageReader<DeathTrigger>
) {
    for (death_trigger) in reader.read() {
        
    }
}