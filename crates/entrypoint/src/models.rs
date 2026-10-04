use std::{
  fs::File,
  io::{self, Read, Write as _},
  path::Path,
};

use rand::{RngExt, rng};
use serde::{Deserialize, Serialize};

use crate::config::Config;

pub static ALPHA: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub static PASSWORD: &[u8] =
  b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_.-!@$%&*+?";

fn generate_random_string(length: usize, charset: &[u8]) -> String {
  let mut rng = rng();

  let result: String = (0..length)
    .map(|_| {
      let idx = rng.random_range(0..charset.len());
      charset[idx] as char
    })
    .collect();

  result
}

// serini quotes values the way a strict INI parser expects (`\;`, `\"`, `key =
// value`), but Project Zomboid reads the file as plain `key=value` lines and
// never unescapes, so the quoting is stripped before the file is written.
fn to_pz_ini(content: &str) -> String {
  let mut out = String::with_capacity(content.len());

  for line in content.lines() {
    let Some((key, value)) = line.split_once('=') else {
      continue;
    };

    out.push_str(key.trim());
    out.push('=');
    out.push_str(
      value
        .trim()
        .replace("\\;", ";")
        .replace("\\#", "#")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
        .as_str(),
    );
    out.push('\n');
  }

  out
}

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

impl Default for ServerConfig {
  fn default() -> Self {
    let mut rng = rng();

    Self {
      pvp: true,
      pvp_log_tool_chat: true,
      pvp_log_tool_file: true,
      pvp_melee_damage_modifier: 30.0,
      pvp_firearm_damage_modifier: 50.0,
      pvp_melee_while_hit_reaction: false,

      pause_empty: true,
      fast_forward_multiplier: 40.0,
      no_fire: false,
      announce_death: false,
      announce_animal_death: false,
      save_world_every_minutes: 0,
      seed: generate_random_string(16, ALPHA),
      use_physics_hit_reaction: false,

      global_chat: true,
      chat_streams: "s,r,a,w,y,sh,f,all".to_string(),
      chat_message_character_limit: 200,
      chat_message_slow_mode_time: 3,
      server_welcome_message: "Welcome to Project Zomboid Multiplayer! <LINE> <LINE> To interact with the Chat panel: press Tab, T, or Enter. <LINE> <LINE> The Tab key will change the target stream of the message. <LINE> <LINE> Global Streams: /all <LINE> Local Streams: /say, /yell <LINE> Special Steams: /whisper, /safehouse, /faction. <LINE> <LINE> Press the Up arrow to cycle through your message history. Click the Gear icon to customize chat. <LINE> <LINE> Happy surviving!".to_string(),
      discord_enable: false,
      discord_token: "".to_string(),
      discord_chat_channel: "".to_string(),
      discord_log_channel: "".to_string(),
      discord_command_channel: "".to_string(),
      webhook_address: "".to_string(),

      server_player_id: rng.random(),
      reset_id: rng.random(),
      public: false,
      public_name: "My PZ Server".to_string(),
      public_description: "".to_string(),
      mods: "".to_string(),
      workshop_items: "".to_string(),
      map: "Muldraugh, KY".to_string(),
      password: "".to_string(),

      default_port: 16261,
      udp_port: 16262,
      ping_limit: 0,
      max_players: 32,
      max_packets_per_second: 300,
      server_browser_announced_ip: "".to_string(),
      rcon_port: 27015,
      rcon_password: generate_random_string(32, PASSWORD),
      do_lua_checksum: true,
      deny_login_on_overloaded_server: true,
      login_queue_enabled: false,
      login_queue_connect_timeout: 60,
      upnp: true,

      safehouse_prevents_loot_respawn: true,
      player_safehouse: false,
      admin_safehouse: false,
      safehouse_allow_trespass: true,
      safehouse_allow_fire: true,
      safehouse_allow_loot: true,
      safehouse_allow_respawn: false,
      safehouse_day_survived_to_claim: 0,
      safe_house_removal_time: 144,
      safehouse_allow_non_residential: false,
      safehouse_disable_disguises: true,
      max_safezone_size: 20000,
      disable_safehouse_when_owner_connected: false,

      display_user_name: true,
      show_first_and_last_name: false,
      username_disguises: false,
      hide_disguised_user_name: false,
      mouse_over_to_see_display_name: true,
      hide_players_behind_you: true,
      ban_kick_global_sound: true,
      trash_delete_all: false,
      player_bump_player: false,
      map_remote_player_visibility: 1,
      allow_non_ascii_username: false,
      allow_coop: true,
      steam_scoreboard: false,
      show_coordinates: false,
      disable_scoreboard: false,
      hide_admins_in_player_list: false,

      spawn_point: "0,0,0".to_string(),
      spawn_items: "".to_string(),
      switch_zombies_ownership_each_update: false,
      player_respawn_with_self: false,
      player_respawn_with_other: false,
      knocked_down_allowed: false,
      sneak_mode_hide_from_other_players: true,
      ultra_speed_does_not_affect_to_animals: false,

      safety_system: true,
      show_safety: true,
      safety_toggle_timer: 2,
      safety_cooldown_timer: 3,
      safety_disconnect_delay: 60,

      voice_enable: true,
      voice_min_distance: 10.0,
      voice_max_distance: 100.0,
      voice_3d: true,
      speed_limit: 70.0,

      faction: true,
      faction_day_survived_to_create: 0,
      faction_players_required_for_tag: 1,

      disable_vehicle_towing: false,
      disable_trailer_towing: false,
      disable_burnt_towing: false,
      car_engine_attraction_modifier: 0.5,

      war: false,
      war_start_delay: 600,
      war_duration: 3600,
      war_safehouse_hit_points: 3,
      allow_destruction_by_sledgehammer: true,
      sledgehammer_only_in_safehouse: false,

      anti_cheat_safety: 2,
      anti_cheat_speed: 2,
      anti_cheat_no_clip: 4,
      anti_cheat_hit: 2,
      anti_cheat_packet_exception: 4,
      anti_cheat_permission: 2,
      anti_cheat_xp: 2,
      anti_cheat_safe_house: 2,
      anti_cheat_player: 2,
      anti_cheat_checksum: 2,

      bad_word_list_file: "".to_string(),
      good_word_list_file: "".to_string(),
      bad_word_policy: 3,
      bad_word_replacement: "[HIDDEN]".to_string(),
      steam_vac: true,
      drop_off_white_list_after_death: false,

      client_command_filter: "-vehicle.*;+vehicle.damageWindow;+vehicle.fixPart;+vehicle.installPart;+vehicle.uninstallPart".to_string(),
      client_action_logs: "ISEnterVehicle;ISExitVehicle;ISTakeEngineParts;".to_string(),
      perk_logs: true,
      item_numbers_limit_per_container: 0,
      blood_splat_lifespan_days: 0,
      remove_player_corpses_on_corpse_removal: false,
    }
  }
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
    self.public_name = cli.server_name.clone();
    self.password = cli.server_password.clone();
    self.default_port = cli.port;
    self.udp_port = cli.steam_port;
    self.rcon_port = cli.rcon_port as u32;
    self.rcon_password = cli.rcon_password.clone();
    self.max_players = cli.max_players;
    self.pause_empty = cli.pause_empty;
    self.steam_vac = cli.steam_vac;
    self.upnp = cli.upnp;
  }
}
