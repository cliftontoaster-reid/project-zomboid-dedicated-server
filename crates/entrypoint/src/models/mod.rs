use std::{
  fs::File,
  io::{self, Read, Write as _},
  path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{config::Config, ini::to_pz_ini};

mod defaults;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct ServerConfig {
  #[serde(rename = "PVP")]
  pub pvp: bool,
  #[serde(rename = "PVPLogToolChat")]
  pub pvp_log_tool_chat: bool,
  #[serde(rename = "PVPLogToolFile")]
  pub pvp_log_tool_file: bool,
  #[serde(rename = "PVPMeleeDamageModifier")]
  pub pvp_melee_damage_modifier: f64,
  #[serde(rename = "PVPFirearmDamageModifier")]
  pub pvp_firearm_damage_modifier: f64,
  #[serde(rename = "PVPMeleeWhileHitReaction")]
  pub pvp_melee_while_hit_reaction: bool,

  pub pause_empty: bool,
  pub fast_forward_multiplier: f64,
  pub no_fire: bool,
  pub announce_death: bool,
  pub announce_animal_death: bool,
  pub save_world_every_minutes: u32,
  pub seed: String,
  pub use_physics_hit_reaction: bool,

  pub global_chat: bool,
  pub chat_streams: String,
  pub chat_message_character_limit: u32,
  pub chat_message_slow_mode_time: u32,
  pub server_welcome_message: String,
  pub discord_enable: bool,
  pub discord_token: String,
  pub discord_chat_channel: String,
  pub discord_log_channel: String,
  pub discord_command_channel: String,
  pub webhook_address: String,

  #[serde(rename = "ServerPlayerID")]
  pub server_player_id: u32,
  #[serde(rename = "ResetID")]
  pub reset_id: u32,
  pub public: bool,
  pub public_name: String,
  pub public_description: String,
  pub mods: String,
  pub workshop_items: String,
  pub map: String,
  pub password: String,

  pub default_port: u16,
  #[serde(rename = "UDPPort")]
  pub udp_port: u16,
  pub ping_limit: u32,
  pub max_players: u32,
  pub max_packets_per_second: u32,
  #[serde(rename = "server_browser_announced_ip")]
  pub server_browser_announced_ip: String,
  #[serde(rename = "RCONPort")]
  pub rcon_port: u32,
  #[serde(rename = "RCONPassword")]
  pub rcon_password: String,
  pub do_lua_checksum: bool,
  pub deny_login_on_overloaded_server: bool,
  pub login_queue_enabled: bool,
  pub login_queue_connect_timeout: u32,
  #[serde(rename = "UPnP")]
  pub upnp: bool,

  pub safehouse_prevents_loot_respawn: bool,
  pub player_safehouse: bool,
  pub admin_safehouse: bool,
  #[serde(rename = "SafehouseAllowTrepass")]
  pub safehouse_allow_trespass: bool,
  pub safehouse_allow_fire: bool,
  pub safehouse_allow_loot: bool,
  pub safehouse_allow_respawn: bool,
  pub safehouse_day_survived_to_claim: u32,
  pub safe_house_removal_time: u32,
  pub safehouse_allow_non_residential: bool,
  pub safehouse_disable_disguises: bool,
  pub max_safezone_size: u32,
  pub disable_safehouse_when_owner_connected: bool,

  pub display_user_name: bool,
  pub show_first_and_last_name: bool,
  pub username_disguises: bool,
  pub hide_disguised_user_name: bool,
  pub mouse_over_to_see_display_name: bool,
  pub hide_players_behind_you: bool,
  pub ban_kick_global_sound: bool,
  pub trash_delete_all: bool,
  pub player_bump_player: bool,
  pub map_remote_player_visibility: u32,
  pub allow_non_ascii_username: bool,
  pub allow_coop: bool,
  pub steam_scoreboard: bool,
  pub show_coordinates: bool,
  pub disable_scoreboard: bool,
  pub hide_admins_in_player_list: bool,

  pub spawn_point: String,
  pub spawn_items: String,
  pub switch_zombies_ownership_each_update: bool,
  pub player_respawn_with_self: bool,
  pub player_respawn_with_other: bool,
  pub knocked_down_allowed: bool,
  pub sneak_mode_hide_from_other_players: bool,
  #[serde(rename = "UltraSpeedDoesnotAffectToAnimals")]
  pub ultra_speed_does_not_affect_to_animals: bool,

  pub safety_system: bool,
  pub show_safety: bool,
  pub safety_toggle_timer: u32,
  pub safety_cooldown_timer: u32,
  pub safety_disconnect_delay: u32,

  pub voice_enable: bool,
  pub voice_min_distance: f64,
  pub voice_max_distance: f64,
  #[serde(rename = "Voice3D")]
  pub voice_3d: bool,
  pub speed_limit: f64,

  pub faction: bool,
  pub faction_day_survived_to_create: u32,
  pub faction_players_required_for_tag: u32,

  pub disable_vehicle_towing: bool,
  pub disable_trailer_towing: bool,
  pub disable_burnt_towing: bool,
  pub car_engine_attraction_modifier: f64,

  pub war: bool,
  pub war_start_delay: u32,
  pub war_duration: u32,
  pub war_safehouse_hit_points: u32,
  pub allow_destruction_by_sledgehammer: bool,
  pub sledgehammer_only_in_safehouse: bool,

  pub anti_cheat_safety: u32,
  pub anti_cheat_speed: u32,
  pub anti_cheat_no_clip: u32,
  pub anti_cheat_hit: u32,
  pub anti_cheat_packet_exception: u32,
  pub anti_cheat_permission: u32,
  #[serde(rename = "AntiCheatXP")]
  pub anti_cheat_xp: u32,
  pub anti_cheat_safe_house: u32,
  pub anti_cheat_player: u32,
  pub anti_cheat_checksum: u32,

  pub bad_word_list_file: String,
  pub good_word_list_file: String,
  pub bad_word_policy: u32,
  pub bad_word_replacement: String,
  #[serde(rename = "SteamVAC")]
  pub steam_vac: bool,
  pub drop_off_white_list_after_death: bool,

  pub client_command_filter: String,
  pub client_action_logs: String,
  pub perk_logs: bool,
  pub item_numbers_limit_per_container: u32,
  pub blood_splat_lifespan_days: u32,
  pub remove_player_corpses_on_corpse_removal: bool,
}

impl ServerConfig {
  pub fn read_config(path: &Path) -> io::Result<ServerConfig> {
    let mut str = String::new();
    File::open(path)?.read_to_string(&mut str)?;

    serini::from_str(&str).map_err(|err| io::Error::new(io::ErrorKind::Other, err))
  }

  pub fn write_config(&self, path: &Path) -> io::Result<()> {
    let content =
      serini::to_string(self).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    let mut file = File::create(path)?;
    file.write_all(to_pz_ini(&content).as_bytes())
  }

  pub fn apply_cli(&mut self, cli: &Config) {
    // These are the entrypoint options that have corresponding Project Zomboid
    // INI keys. Runtime-only options such as paths, memory, and UID/GID are
    // intentionally handled outside the server configuration.
    if let Some(value) = &cli.server_name {
      self.public_name = value.clone();
    }
    if let Some(value) = &cli.server_password {
      self.password = value.clone();
    }
    if let Some(value) = cli.port {
      self.default_port = value;
    }
    if let Some(value) = cli.steam_port {
      self.udp_port = value;
    }
    if let Some(value) = cli.rcon_port {
      self.rcon_port = value as u32;
    }
    if let Some(value) = &cli.rcon_password {
      self.rcon_password = value.clone();
    }
    if let Some(value) = cli.max_players {
      self.max_players = value;
    }
    if let Some(value) = cli.pause_empty {
      self.pause_empty = value;
    }
    if let Some(value) = cli.steam_vac {
      self.steam_vac = value;
    }
    if let Some(value) = cli.upnp {
      self.upnp = value;
    }
  }
}
