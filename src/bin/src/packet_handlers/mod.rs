pub(crate) mod play_packets;
pub mod player;

pub fn register_player_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(player::head_rot::handle_player_move);
    schedule.add_systems(player::send_inventory_updates::handle_inventory_updates);
    schedule.add_systems(player::player_disconnect::handle);
}
