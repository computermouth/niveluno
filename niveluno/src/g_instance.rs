use crate::d_computer::Computer;
use crate::d_floor::Floor;
use crate::d_platform::Platform;
use crate::d_table::Table;

use crate::e_barrier::Barrier;
use crate::e_gcyl::Gcyl;
use crate::e_light::Light;
use crate::e_menu::Menu;
use crate::e_pig::Pig;
use crate::e_player::Player;

use crate::g_game;
use crate::map::{Entity, LoadedEnttReference};
use crate::nuerror::NUError;

use raymath::{vector3_distance, vector3_normalize, vector3_subtract, Vector3};

use crate::math;

pub enum Instance {
    // Decor
    DComputer(Computer),
    DFloor(Floor),
    DPlatform(Platform),
    DTable(Table),
    // Entities
    EBarrier(Barrier),
    EGcyl(Gcyl),
    ELight(Light),
    EPlayer(Player),
    EPig(Pig),
    EMenuM(Menu),
    EMenuE(Menu),
    EMenuN(Menu),
    EMenuU(Menu),
    ETriggerLevelChange, // todo
}

// todo -- perf
// cache these lookups, probably perform the cache at map_load
pub fn ref_ent_from_str(s: &str) -> Option<LoadedEnttReference> {
    let ents = g_game::get_map_ref_ents().unwrap();
    let erns = g_game::get_map_ern_data().unwrap();

    let mut out = None;
    for (i, ern) in erns.iter().enumerate() {
        if s == ern {
            out = Some(ents[i].clone())
        }
    }

    out
}

pub fn instance_from_str(s: &str, entt: &Entity) -> Option<Instance> {
    match s {
        // decor

		// PLATFORMER
		"platformer.ball" |
		"platformer.barrier_1x1x1" |
		"platformer.barrier_1x1x2" |
		"platformer.barrier_1x1x4" |
		"platformer.barrier_2x1x1" |
		"platformer.barrier_2x1x2" |
		"platformer.barrier_2x1x4" |
		"platformer.barrier_3x1x1" |
		"platformer.barrier_3x1x2" |
		"platformer.barrier_3x1x4" |
		"platformer.barrier_4x1x1" |
		"platformer.barrier_4x1x2" |
		"platformer.barrier_4x1x4" |
		"platformer.bomb" |
		"platformer.cannon_bullet" |
		"platformer.chain_full" |
		"platformer.chain_link" |
		"platformer.chain_link_end_bottom" |
		"platformer.chain_link_end_top" |
		"platformer.cone" |
		"platformer.floor_spikes_2x2x1" |
		"platformer.floor_spikes_4x4x1" |
		"platformer.floor_spikes_curved_4x2x2" |
		"platformer.floor_wood_1x1" |
		"platformer.floor_wood_2x2" |
		"platformer.floor_wood_2x6" |
		"platformer.floor_wood_4x4" |
		"platformer.hammerblock" |
		"platformer.hammerblock_spikes" |
		"platformer.pillar_1x1x1" |
		"platformer.pillar_1x1x2" |
		"platformer.pillar_1x1x4" |
		"platformer.pillar_1x1x8" |
		"platformer.pillar_2x2x2" |
		"platformer.pillar_2x2x4" |
		"platformer.pillar_2x2x8" |
		"platformer.platform_wood_1x1x1" |
		"platformer.sawblade" |
		"platformer.sign" |
		"platformer.signage_arrows_left" |
		"platformer.signage_arrows_right" |
		"platformer.signage_finish" |
		"platformer.signage_finish_wide" |
		"platformer.spikeball" |
		"platformer.spikeball_hanger" |
		"platformer.spikeroller_horizontal" |
		"platformer.spikeroller_vertical" |
		"platformer.spring" |
		"platformer.structure_A" |
		"platformer.structure_B" |
		"platformer.structure_C" |
		"platformer.strut_horizontal" |
		"platformer.strut_vertical" |
		// PLATFORMER_BLUE
		"platformer.arch_blue" |
		"platformer.arch_tall_blue" |
		"platformer.arch_wide_blue" |
		"platformer.ball_blue" |
		"platformer.barrier_1x1x1_blue" |
		"platformer.barrier_1x1x2_blue" |
		"platformer.barrier_1x1x4_blue" |
		"platformer.barrier_2x1x1_blue" |
		"platformer.barrier_2x1x2_blue" |
		"platformer.barrier_2x1x4_blue" |
		"platformer.barrier_3x1x1_blue" |
		"platformer.barrier_3x1x2_blue" |
		"platformer.barrier_3x1x4_blue" |
		"platformer.barrier_4x1x1_blue" |
		"platformer.barrier_4x1x2_blue" |
		"platformer.barrier_4x1x4_blue" |
		"platformer.bomb_A_blue" |
		"platformer.bomb_B_blue" |
		"platformer.bracing_large_blue" |
		"platformer.bracing_medium_blue" |
		"platformer.bracing_small_blue" |
		"platformer.button_base_blue" |
		"platformer.cannon_base_blue" |
		"platformer.chest_blue" |
		"platformer.chest_large_blue" |
		"platformer.cone_blue" |
		"platformer.conveyor_2x4x1_blue" |
		"platformer.conveyor_2x8x1_blue" |
		"platformer.conveyor_4x4x1_blue" |
		"platformer.conveyor_4x8x1_blue" |
		"platformer.diamond_blue" |
		"platformer.flag_A_blue" |
		"platformer.flag_B_blue" |
		"platformer.flag_C_blue" |
		"platformer.floor_net_2x2x1_blue" |
		"platformer.floor_net_4x4x1_blue" |
		"platformer.floor_spikes_trap_2x2x1_blue_empty" |
		"platformer.floor_spikes_trap_4x4x1_blue" |
		"platformer.floor_spikes_trap_4x4x1_blue_empty" |
		"platformer.floor_spikes_trap_spikes_2x2x1" |
		"platformer.floor_spikes_trap_spikes_2x2x1_blue" |
		"platformer.floor_spikes_trap_spikes_4x4x1" |
		"platformer.hammer_blue" |
		"platformer.hammer_large_blue" |
		"platformer.hammer_large_spikes_blue" |
		"platformer.hammer_spikes_blue" |
		"platformer.heart_blue" |
		"platformer.hoop_angled_blue" |
		"platformer.hoop_blue" |
		"platformer.lever_floor_base_blue" |
		"platformer.lever_wall_base_A_blue" |
		"platformer.lever_wall_base_B_blue" |
		"platformer.pipe_90_A_blue" |
		"platformer.pipe_90_B_blue" |
		"platformer.pipe_180_A_blue" |
		"platformer.pipe_180_B_blue" |
		"platformer.pipe_end_blue" |
		"platformer.pipe_straight_A_blue" |
		"platformer.pipe_straight_B_blue" |
		"platformer.platform_1x1x1_blue" |
		"platformer.platform_2x2x1_blue" |
		"platformer.platform_2x2x2_blue" |
		"platformer.platform_2x2x4_blue" |
		"platformer.platform_4x2x1_blue" |
		"platformer.platform_4x2x2_blue" |
		"platformer.platform_4x2x4_blue" |
		"platformer.platform_4x4x1_blue" |
		"platformer.platform_4x4x2_blue" |
		"platformer.platform_4x4x4_blue" |
		"platformer.platform_6x2x1_blue" |
		"platformer.platform_6x2x2_blue" |
		"platformer.platform_6x2x4_blue" |
		"platformer.platform_6x6x1_blue" |
		"platformer.platform_6x6x2_blue" |
		"platformer.platform_6x6x4_blue" |
		"platformer.platform_arrow_2x2x1_blue" |
		"platformer.platform_arrow_4x4x1_blue" |
		"platformer.platform_decorative_1x1x1_blue" |
		"platformer.platform_decorative_2x2x2_blue" |
		"platformer.platform_hole_6x6x1_blue" |
		"platformer.platform_slope_2x2x2_blue" |
		"platformer.platform_slope_2x4x4_blue" |
		"platformer.platform_slope_2x6x4_blue" |
		"platformer.platform_slope_4x2x2_blue" |
		"platformer.platform_slope_4x4x4_blue" |
		"platformer.platform_slope_4x6x4_blue" |
		"platformer.platform_slope_6x2x2_blue" |
		"platformer.platform_slope_6x4x4_blue" |
		"platformer.platform_slope_6x6x4_blue" |
		"platformer.power_blue" |
		"platformer.railing_corner_double_blue" |
		"platformer.railing_corner_padded_blue" |
		"platformer.railing_corner_single_blue" |
		"platformer.railing_straight_double_blue" |
		"platformer.railing_straight_padded_blue" |
		"platformer.railing_straight_single_blue" |
		"platformer.safetynet_2x2x1_blue" |
		"platformer.safetynet_4x2x1_blue" |
		"platformer.safetynet_6x2x1_blue" |
		"platformer.saw_trap_blue" |
		"platformer.saw_trap_double_blue" |
		"platformer.saw_trap_long_blue" |
		"platformer.signage_arrow_stand_blue" |
		"platformer.signage_arrow_wall_blue" |
		"platformer.signage_arrows_left_blue" |
		"platformer.signage_arrows_right_blue" |
		"platformer.spikeblock_double_horizontal_blue" |
		"platformer.spikeblock_double_vertical_blue" |
		"platformer.spikeblock_down_blue" |
		"platformer.spikeblock_left_blue" |
		"platformer.spikeblock_omni_blue" |
		"platformer.spikeblock_quad_blue" |
		"platformer.spikeblock_right_blue" |
		"platformer.spikeblock_up_blue" |
		"platformer.spring_pad_blue" |
		"platformer.star_blue" |
		"platformer.swiper_blue" |
		"platformer.swiper_double_blue" |
		"platformer.swiper_double_long_blue" |
		"platformer.swiper_long_blue" |
		"platformer.swiper_quad_blue" |
		"platformer.swiper_quad_long_blue" |
		// MEDIEVAL
		"medieval.building_archeryrange_blue" |
		"medieval.building_barracks_blue" |
		"medieval.building_blacksmith_blue" |
		"medieval.building_castle_blue" |
		"medieval.building_church_blue" |
		"medieval.building_docks_blue" |
		"medieval.building_home_A_blue" |
		"medieval.building_home_B_blue" |
		"medieval.building_lumbermill_blue" |
		"medieval.building_market_blue" |
		"medieval.building_mine_blue" |
		"medieval.building_shipyard_blue" |
		"medieval.building_shrine_blue" |
		"medieval.building_stables_blue" |
		"medieval.building_tavern_blue" |
		"medieval.building_tent_blue" |
		"medieval.building_tower_A_blue" |
		"medieval.building_tower_A_blue_top" |
		"medieval.building_tower_B_blue" |
		"medieval.building_tower_B_blue_top" |
		"medieval.building_tower_base_blue" |
		"medieval.building_tower_cannon_blue" |
		"medieval.building_tower_catapult_blue" |
		"medieval.building_townhall_blue" |
		"medieval.building_watchtower_blue" |
		"medieval.building_watermill_blue" |
		"medieval.building_watermill_blue_no_wheel" |
		"medieval.building_watermill_wheel" |
		"medieval.building_well_blue" |
		"medieval.building_windmill_blue" |
		"medieval.building_windmill_bot_blue" |
		"medieval.building_windmill_top_blue" |
		"medieval.building_windmill_top_fan" |
		"medieval.building_workshop_blue" |
		"medieval.cannon_blue" |
		"medieval.cannon_turret_blue" |
		"medieval.catapult_arm_blue" |
		"medieval.catapult_turret_blue" |
		"medieval.cloud_big" |
		"medieval.cloud_small" |
		"medieval.hill_single_A" |
		"medieval.hill_single_B" |
		"medieval.hill_single_C" |
		"medieval.hills_A" |
		"medieval.hills_A_trees" |
		"medieval.hills_B" |
		"medieval.hills_B_trees" |
		"medieval.hills_C" |
		"medieval.hills_C_trees" |
		"medieval.mountain_A" |
		"medieval.mountain_A_grass" |
		"medieval.mountain_A_grass_trees" |
		"medieval.mountain_B" |
		"medieval.mountain_B_grass" |
		"medieval.mountain_B_grass_trees" |
		"medieval.mountain_C" |
		"medieval.mountain_C_grass" |
		"medieval.mountain_C_grass_trees" |
		"medieval.rock_single_A" |
		"medieval.rock_single_B" |
		"medieval.rock_single_C" |
		"medieval.rock_single_D" |
		"medieval.rock_single_E" |
		"medieval.tree_single_A" |
		"medieval.tree_single_A_cut" |
		"medieval.tree_single_B" |
		"medieval.tree_single_B_cut" |
		"medieval.trees_A_cut" |
		"medieval.trees_A_large" |
		"medieval.trees_A_medium" |
		"medieval.trees_A_small" |
		"medieval.trees_B_cut" |
		"medieval.trees_B_large" |
		"medieval.trees_B_medium" |
		"medieval.trees_B_small" |
		"medieval.waterlily_A" |
		"medieval.waterlily_B" |
		"medieval.waterplant_A" |
		"medieval.waterplant_B" |
		"medieval.waterplant_C" |
		"medieval.anchor" |
		"medieval.barrel" |
		"medieval.boat" |
		"medieval.boatrack" |
		"medieval.bucket_arrows" |
		"medieval.bucket_empty" |
		"medieval.bucket_water" |
		"medieval.cannonball_pallet" |
		"medieval.crate_A_big" |
		"medieval.crate_A_small" |
		"medieval.crate_B_big" |
		"medieval.crate_B_small" |
		"medieval.crate_long_A" |
		"medieval.crate_long_B" |
		"medieval.crate_long_C" |
		"medieval.crate_long_empty" |
		"medieval.crate_open" |
		"medieval.flag_blue" |
		"medieval.flag_green" |
		"medieval.flag_red" |
		"medieval.flag_yellow" |
		"medieval.haybale" |
		"medieval.icon_combat" |
		"medieval.icon_range" |
		"medieval.ladder" |
		"medieval.pallet" |
		"medieval.resource_lumber" |
		"medieval.resource_stone" |
		"medieval.sack" |
		"medieval.target" |
		"medieval.tent" |
		"medieval.trough" |
		"medieval.trough_long" |
		"medieval.weaponrack" |
		"medieval.wheelbarrow" |
		"medieval.hex_grass" |
		"medieval.hex_grass_bottom" |
		"medieval.hex_grass_sloped_high" |
		"medieval.hex_grass_sloped_low" |
		"medieval.hex_transition" |
		"medieval.hex_water" |
		"medieval.hex_coast_A" |
		"medieval.hex_coast_B" |
		"medieval.hex_coast_C" |
		"medieval.hex_coast_D" |
		"medieval.hex_coast_E" |
		"medieval.hex_coast_A_waterless" |
		"medieval.hex_coast_B_waterless" |
		"medieval.hex_coast_C_waterless" |
		"medieval.hex_coast_D_waterless" |
		"medieval.hex_coast_E_waterless" |
		"medieval.hex_river_A" |
		"medieval.hex_river_A_curvy" |
		"medieval.hex_river_B" |
		"medieval.hex_river_C" |
		"medieval.hex_river_crossing_A" |
		"medieval.hex_river_crossing_B" |
		"medieval.hex_river_D" |
		"medieval.hex_river_E" |
		"medieval.hex_river_F" |
		"medieval.hex_river_G" |
		"medieval.hex_river_H" |
		"medieval.hex_river_I" |
		"medieval.hex_river_J" |
		"medieval.hex_river_K" |
		"medieval.hex_river_L" |
		"medieval.hex_river_A_curvy_waterless" |
		"medieval.hex_river_A_waterless" |
		"medieval.hex_river_B_waterless" |
		"medieval.hex_river_C_waterless" |
		"medieval.hex_river_crossing_A_waterless" |
		"medieval.hex_river_crossing_B_waterless" |
		"medieval.hex_river_D_waterless" |
		"medieval.hex_river_E_waterless" |
		"medieval.hex_river_F_waterless" |
		"medieval.hex_river_G_waterless" |
		"medieval.hex_river_H_waterless" |
		"medieval.hex_river_I_waterless" |
		"medieval.hex_river_J_waterless" |
		"medieval.hex_river_K_waterless" |
		"medieval.hex_river_L_waterless" |
		"medieval.hex_road_A" |
		"medieval.hex_road_A_sloped_high" |
		"medieval.hex_road_A_sloped_low" |
		"medieval.hex_road_B" |
		"medieval.hex_road_C" |
		"medieval.hex_road_D" |
		"medieval.hex_road_E" |
		"medieval.hex_road_F" |
		"medieval.hex_road_G" |
		"medieval.hex_road_H" |
		"medieval.hex_road_I" |
		"medieval.hex_road_J" |
		"medieval.hex_road_K" |
		"medieval.hex_road_L" |
		"medieval.hex_road_M" |
		"medieval.banner_blue_accent" |
		"medieval.banner_blue_full" |
		"medieval.bow_blue_accent" |
		"medieval.bow_blue_full" |
		"medieval.helmet_blue_accent" |
		"medieval.helmet_blue_full" |
		"medieval.horse_blue_accent" |
		"medieval.horse_blue_full" |
		"medieval.projectile_arrow_blue_accent" |
		"medieval.projectile_arrow_blue_full" |
		"medieval.shield_blue_accent" |
		"medieval.shield_blue_full" |
		"medieval.ship_blue_accent" |
		"medieval.ship_blue_full" |
		"medieval.spear_blue_accent" |
		"medieval.spear_blue_full" |
		"medieval.sword_blue_accent" |
		"medieval.sword_blue_full" |
		"medieval.unit_blue_accent" |
		"medieval.unit_blue_full" |
		"medieval.cannon_blue_accent" |
		"medieval.cannon_blue_full" |
		"medieval.cart_blue_accent" |
		"medieval.cart_blue_full" |
		"medieval.cart_merchant_blue_accent" |
		"medieval.cart_merchant_blue_full" |
		"medieval.catapult_blue_accent" |
		"medieval.catapult_blue_full" |
		// EGGSMAS
		"xmas.basketball" |
		"xmas.bell" |
		"xmas.bell_decorated" |
		"xmas.candy_A_blue" |
		"xmas.candy_A_green" |
		"xmas.candy_A_pink" |
		"xmas.candy_A_yellow" |
		"xmas.candy_B_blue" |
		"xmas.candy_B_green" |
		"xmas.candy_B_pink" |
		"xmas.candy_B_yellow" |
		"xmas.candy_C_blue" |
		"xmas.candy_C_green" |
		"xmas.candy_C_pink" |
		"xmas.candy_C_yellow" |
		"xmas.candy_peppermint" |
		"xmas.candycane_large" |
		"xmas.candycane_small" |
		"xmas.carpet_round_large" |
		"xmas.carpet_round_small" |
		"xmas.chair_large_blue" |
		"xmas.chair_large_brown" |
		"xmas.chair_large_green" |
		"xmas.chair_large_red" |
		"xmas.christmas_tree" |
		"xmas.christmas_tree_base" |
		"xmas.christmas_tree_decorated" |
		"xmas.christmas_tree_withoutLights" |
		"xmas.cookie" |
		"xmas.cube_gingerbread_large_A" |
		"xmas.cube_gingerbread_large_B" |
		"xmas.cube_gingerbread_slope" |
		"xmas.cube_gingerbread_small_A" |
		"xmas.cube_gingerbread_small_B" |
		"xmas.door_gingerbread" |
		"xmas.floor_gingerbread_large" |
		"xmas.floor_gingerbread_small" |
		"xmas.football" |
		"xmas.footstool_blue" |
		"xmas.footstool_brown" |
		"xmas.footstool_green" |
		"xmas.footstool_red" |
		"xmas.gingerbread_house" |
		"xmas.gingerbread_house_decorated" |
		"xmas.gingerbread_man" |
		"xmas.hot_chocolate" |
		"xmas.hot_chocolate_decorated" |
		"xmas.lantern" |
		"xmas.lantern_decorated" |
		"xmas.lantern_mini" |
		"xmas.milk" |
		"xmas.mistletoe_A" |
		"xmas.mistletoe_B" |
		"xmas.pillar_gingerbread_large_A" |
		"xmas.pillar_gingerbread_large_B" |
		"xmas.pillar_gingerbread_small_A" |
		"xmas.pillar_gingerbread_small_B" |
		"xmas.plate_blue" |
		"xmas.plate_decorated_A" |
		"xmas.plate_decorated_B" |
		"xmas.plate_red" |
		"xmas.plate_small_blue" |
		"xmas.plate_small_red" |
		"xmas.plate_small_white" |
		"xmas.plate_white" |
		"xmas.present_A_blue" |
		"xmas.present_A_green" |
		"xmas.present_A_red" |
		"xmas.present_A_white" |
		"xmas.present_A_yellow" |
		"xmas.present_B_blue" |
		"xmas.present_B_green" |
		"xmas.present_B_red" |
		"xmas.present_B_white" |
		"xmas.present_B_yellow" |
		"xmas.present_C_blue" |
		"xmas.present_C_green" |
		"xmas.present_C_red" |
		"xmas.present_C_white" |
		"xmas.present_C_yellow" |
		"xmas.present_D_blue" |
		"xmas.present_D_green" |
		"xmas.present_D_red" |
		"xmas.present_D_white" |
		"xmas.present_D_yellow" |
		"xmas.present_E_blue" |
		"xmas.present_E_green" |
		"xmas.present_E_red" |
		"xmas.present_E_white" |
		"xmas.present_E_yellow" |
		"xmas.present_F_blue" |
		"xmas.present_F_green" |
		"xmas.present_F_red" |
		"xmas.present_F_white" |
		"xmas.present_F_yellow" |
		"xmas.present_sphere_A_blue" |
		"xmas.present_sphere_A_green" |
		"xmas.present_sphere_A_red" |
		"xmas.present_sphere_A_white" |
		"xmas.present_sphere_A_yellow" |
		"xmas.present_sphere_B_blue" |
		"xmas.present_sphere_B_green" |
		"xmas.present_sphere_B_red" |
		"xmas.present_sphere_B_white" |
		"xmas.present_sphere_B_yellow" |
		"xmas.roof_gingerbread_left" |
		"xmas.roof_gingerbread_left_overhang" |
		"xmas.roof_gingerbread_right" |
		"xmas.roof_gingerbread_right" |
		"xmas.snowball" |
		"xmas.snowball_cannon" |
		"xmas.snowball_pile" |
		"xmas.snowman_A" |
		"xmas.snowman_B" |
		"xmas.stool" |
		"xmas.tracks_crossing" |
		"xmas.tracks_curve" |
		"xmas.tracks_split_left" |
		"xmas.tracks_split_right" |
		"xmas.tracks_straight" |
		"xmas.tracks_straight_station" |
		"xmas.train_locomotive" |
		"xmas.train_tender_coal" |
		"xmas.train_tender_empty" |
		"xmas.train_tender_presents_A" |
		"xmas.train_tender_presents_B" |
		"xmas.train_wagon" |
		"xmas.wall_decoration_candy_A" |
		"xmas.wall_decoration_candy_B" |
		"xmas.wall_decoration_candy_C" |
		"xmas.wall_gingerbread_A" |
		"xmas.wall_gingerbread_B" |
		"xmas.wall_gingerbread_doorway_A" |
		"xmas.wall_gingerbread_doorway_B" |
		"xmas.wall_gingerbread_slope" |
		"xmas.wall_gingerbread_window_A" |
		"xmas.wall_gingerbread_window_B" |
		"xmas.wreath" |
		"xmas.tracks_split_arrow" |
		// HALLOWEEN
		"halloween.arch" |
		"halloween.arch_gate" |
		"halloween.arch_gate_closed" |
		"halloween.arch_gate_left" |
		"halloween.arch_gate_open" |
		"halloween.arch_gate_right" |
		"halloween.bench" |
		"halloween.bench_decorated" |
		"halloween.bone_A" |
		"halloween.bone_B" |
		"halloween.bone_C" |
		"halloween.candle" |
		"halloween.candle_melted" |
		"halloween.candle_thin" |
		"halloween.candle_triple" |
		"halloween.candy_blue_A" |
		"halloween.candy_blue_B" |
		"halloween.candy_brown_A" |
		"halloween.candy_brown_B" |
		"halloween.candy_brown_C" |
		"halloween.candy_bucket_A_decorated_no_handle" |
		"halloween.candy_bucket_A_decorated_w_handle" |
		"halloween.candy_bucket_A_no_handle" |
		"halloween.candy_bucket_A_w_handle" |
		"halloween.candy_bucket_B_decorated_no_handle" |
		"halloween.candy_bucket_B_decorated_w_handle" |
		"halloween.candy_bucket_B_no_handle" |
		"halloween.candy_bucket_B_w_handle" |
		"halloween.candy_green_A" |
		"halloween.candy_green_B" |
		"halloween.candy_green_C" |
		"halloween.candy_orange_A" |
		"halloween.candy_orange_B" |
		"halloween.candy_orange_C" |
		"halloween.candy_pink_A" |
		"halloween.candy_pink_B" |
		"halloween.candy_purple_A" |
		"halloween.candy_purple_B" |
		"halloween.candycorn" |
		"halloween.coffin_bottom" |
		"halloween.coffin_closed" |
		"halloween.coffin_decorated" |
		"halloween.coffin_top" |
		"halloween.crypt" |
		"halloween.fence" |
		"halloween.fence_broken" |
		"halloween.fence_gate_closed" |
		"halloween.fence_gate_empty" |
		"halloween.fence_gate_open" |
		"halloween.fence_pillar" |
		"halloween.fence_pillar_broken" |
		"halloween.fence_seperate" |
		"halloween.fence_seperate_broken" |
		"halloween.floor_dirt" |
		"halloween.floor_dirt_grave" |
		"halloween.floor_dirt_small" |
		"halloween.grave_A" |
		"halloween.grave_A_destroyed" |
		"halloween.grave_B" |
		"halloween.gravemarker_A" |
		"halloween.gravemarker_B" |
		"halloween.gravestone" |
		"halloween.haybale" |
		"halloween.lantern_hanging" |
		"halloween.lantern_standing" |
		"halloween.lollipop_blue" |
		"halloween.lollipop_brown" |
		"halloween.lollipop_green" |
		"halloween.lollipop_orange" |
		"halloween.lollipop_pink" |
		"halloween.lollipop_purple" |
		"halloween.maze_short" |
		"halloween.maze_tall" |
		"halloween.path_A" |
		"halloween.path_B" |
		"halloween.path_C" |
		"halloween.path_D" |
		"halloween.pillar" |
		"halloween.pitchfork" |
		"halloween.plaque" |
		"halloween.plaque_candles" |
		"halloween.post" |
		"halloween.post_lantern" |
		"halloween.post_skull" |
		"halloween.pumpkin_orange" |
		"halloween.pumpkin_orange_jackolantern" |
		"halloween.pumpkin_orange_small" |
		"halloween.pumpkin_yellow" |
		"halloween.pumpkin_yellow_jackolantern" |
		"halloween.pumpkin_yellow_small" |
		"halloween.ribcage" |
		"halloween.scarecrow" |
		"halloween.shrine" |
		"halloween.shrine_candles" |
		"halloween.sign_both" |
		"halloween.sign_left" |
		"halloween.sign_right" |
		"halloween.skull" |
		"halloween.skull_candle" |
		"halloween.tractor" |
		"halloween.tree_dead_large" |
		"halloween.tree_dead_large_decorated" |
		"halloween.tree_dead_medium" |
		"halloween.tree_dead_small" |
		"halloween.tree_pine_orange_large" |
		"halloween.tree_pine_orange_medium" |
		"halloween.tree_pine_orange_small" |
		"halloween.tree_pine_yellow_large" |
		"halloween.tree_pine_yellow_medium" |
		"halloween.tree_pine_yellow_small" |
		"halloween.wagon" |
		"halloween.wagon_hay" |
		"halloween.wooden_gate" |
		"halloween.wooden_gate_halloween" |
		"halloween.tractor_front_wheel" |
		"halloween.tractor_rear_wheel" |
		"halloween.tractor_steeringwheel" |
		"halloween.wagon_wheel" |
        // FURNITURE
		"furniture.armchair" |
		"furniture.armchair_pillows" |
		"furniture.bed_double_A" |
		"furniture.bed_double_B" |
		"furniture.bed_single_A" |
		"furniture.bed_single_B" |
		"furniture.book_set" |
		"furniture.book_single" |
		"furniture.cabinet_medium" |
		"furniture.cabinet_medium_decorated" |
		"furniture.cabinet_small" |
		"furniture.cabinet_small_decorated" |
		"furniture.cactus_medium_A" |
		"furniture.cactus_medium_B" |
		"furniture.cactus_small_A" |
		"furniture.cactus_small_B" |
		"furniture.chair_A" |
		"furniture.chair_A_wood" |
		"furniture.chair_B" |
		"furniture.chair_B_wood" |
		"furniture.chair_C" |
		"furniture.chair_desk_A" |
		"furniture.chair_desk_B" |
		"furniture.chair_stool" |
		"furniture.chair_stool_wood" |
		"furniture.couch" |
		"furniture.couch_pillows" |
		"furniture.cup" |
		"furniture.cup_pencils" |
		"furniture.desk" |
		"furniture.desk_decorated" |
		"furniture.desk_large" |
		"furniture.desk_large_decorated" |
		"furniture.gameconsole_handheld" |
		"furniture.keyboard" |
		"furniture.lamp_desk" |
		"furniture.lamp_desk_headphones" |
		"furniture.lamp_standing" |
		"furniture.lamp_table" |
		"furniture.monitor" |
		"furniture.mouse" |
		"furniture.mousepad_A" |
		"furniture.mousepad_B" |
		"furniture.mousepad_large_A" |
		"furniture.mousepad_large_B" |
		"furniture.mug_A" |
		"furniture.mug_B" |
		"furniture.pictureframe_large_A" |
		"furniture.pictureframe_large_B" |
		"furniture.pictureframe_medium" |
		"furniture.pictureframe_small_A" |
		"furniture.pictureframe_small_B" |
		"furniture.pictureframe_small_C" |
		"furniture.pictureframe_standing_A" |
		"furniture.pictureframe_standing_B" |
		"furniture.pillow_A" |
		"furniture.pillow_B" |
		"furniture.rug_oval_A" |
		"furniture.rug_oval_B" |
		"furniture.rug_rectangle_A" |
		"furniture.rug_rectangle_B" |
		"furniture.rug_rectangle_stripes_A" |
		"furniture.rug_rectangle_stripes_B" |
		"furniture.shelf_A_big" |
		"furniture.shelf_A_small" |
		"furniture.shelf_B_large" |
		"furniture.shelf_B_large_decorated" |
		"furniture.shelf_B_small" |
		"furniture.shelf_B_small_decorated" |
		"furniture.table_low" |
		"furniture.table_low_decorated" |
		"furniture.table_medium" |
		"furniture.table_medium_long" |
		"furniture.table_small" |
        // FOREST
		"forest.Bush_1_A_Color1" |
		"forest.Bush_1_B_Color1" |
		"forest.Bush_1_C_Color1" |
		"forest.Bush_1_D_Color1" |
		"forest.Bush_1_E_Color1" |
		"forest.Bush_1_F_Color1" |
		"forest.Bush_1_G_Color1" |
		"forest.Bush_2_A_Color1" |
		"forest.Bush_2_B_Color1" |
		"forest.Bush_2_C_Color1" |
		"forest.Bush_2_D_Color1" |
		"forest.Bush_2_E_Color1" |
		"forest.Bush_2_F_Color1" |
		"forest.Bush_3_A_Color1" |
		"forest.Bush_3_B_Color1" |
		"forest.Bush_3_C_Color1" |
		"forest.Bush_4_A_Color1" |
		"forest.Bush_4_B_Color1" |
		"forest.Bush_4_C_Color1" |
		"forest.Bush_4_D_Color1" |
		"forest.Bush_4_E_Color1" |
		"forest.Bush_4_F_Color1" |
		"forest.Grass_1_A_Color1" |
		"forest.Grass_1_A_Singlesided_Color1" |
		"forest.Grass_1_B_Color1" |
		"forest.Grass_1_B_Singlesided_Color1" |
		"forest.Grass_1_C_Color1" |
		"forest.Grass_1_C_Singlesided_Color1" |
		"forest.Grass_1_D_Color1" |
		"forest.Grass_1_D_Singlesided_Color1" |
		"forest.Grass_2_A_Color1" |
		"forest.Grass_2_A_Singlesided_Color1" |
		"forest.Grass_2_B_Color1" |
		"forest.Grass_2_B_Singlesided_Color1" |
		"forest.Grass_2_C_Color1" |
		"forest.Grass_2_C_Singlesided_Color1" |
		"forest.Grass_2_D_Color1" |
		"forest.Grass_2_D_Singlesided_Color1" |
		"forest.Hill_2x2x2_Color1" |
		"forest.Hill_2x2x4_Color1" |
		"forest.Hill_2x2x8_Color1" |
		"forest.Hill_4x2x2_Color1" |
		"forest.Hill_4x2x4_Color1" |
		"forest.Hill_4x2x8_Color1" |
		"forest.Hill_4x4x2_Color1" |
		"forest.Hill_4x4x4_Color1" |
		"forest.Hill_4x4x8_Color1" |
		"forest.Hill_8x4x2_Color1" |
		"forest.Hill_8x4x4_Color1" |
		"forest.Hill_8x4x8_Color1" |
		"forest.Hill_8x8x2_Color1" |
		"forest.Hill_8x8x4_Color1" |
		"forest.Hill_8x8x8_Color1" |
		"forest.Hill_12x6x2_Color1" |
		"forest.Hill_12x6x4_Color1" |
		"forest.Hill_12x6x8_Color1" |
		"forest.Hill_12x12x2_Color1" |
		"forest.Hill_12x12x4_Color1" |
		"forest.Hill_12x12x8_Color1" |
		"forest.Hill_Cliff_A_InnerCorner_Color1" |
		"forest.Hill_Cliff_A_OuterCorner_Color1" |
		"forest.Hill_Cliff_B_Side_Color1" |
		"forest.Hill_Cliff_C_InnerCorner_Color1" |
		"forest.Hill_Cliff_C_OuterCorner_Color1" |
		"forest.Hill_Cliff_D_Side_Color1" |
		"forest.Hill_Cliff_E_Color1" |
		"forest.Hill_Cliff_F_Side_Color1" |
		"forest.Hill_Cliff_G_InnerCorner_Color1" |
		"forest.Hill_Cliff_G_OuterCorner_Color1" |
		"forest.Hill_Cliff_H_Side_Color1" |
		"forest.Hill_Cliff_I_InnerCorner_Color1" |
		"forest.Hill_Cliff_I_OuterCorner_Color1" |
		"forest.Hill_Cliff_Tall_A_InnerCorner_Color1" |
		"forest.Hill_Cliff_Tall_A_OuterCorner_Color1" |
		"forest.Hill_Cliff_Tall_B_Side_Color1" |
		"forest.Hill_Cliff_Tall_C_InnerCorner_Color1" |
		"forest.Hill_Cliff_Tall_C_OuterCorner_Color1" |
		"forest.Hill_Cliff_Tall_D_Side_Color1" |
		"forest.Hill_Cliff_Tall_E_Color1" |
		"forest.Hill_Cliff_Tall_F_Side_Color1" |
		"forest.Hill_Cliff_Tall_G_InnerCorner_Color1" |
		"forest.Hill_Cliff_Tall_G_OuterCorner_Color1" |
		"forest.Hill_Cliff_Tall_H_Side_Color1" |
		"forest.Hill_Cliff_Tall_I_InnerCorner_Color1" |
		"forest.Hill_Cliff_Tall_I_OuterCorner_Color1" |
		"forest.Hill_Top_A_InnerCorner_Color1" |
		"forest.Hill_Top_A_OuterCorner_Color1" |
		"forest.Hill_Top_B_Side_Color1" |
		"forest.Hill_Top_C_InnerCorner_Color1" |
		"forest.Hill_Top_C_OuterCorner_Color1" |
		"forest.Hill_Top_D_Side_Color1" |
		"forest.Hill_Top_E_Cap_Color1" |
		"forest.Hill_Top_E_Center_Color1" |
		"forest.Hill_Top_F_Side_Color1" |
		"forest.Hill_Top_G_InnerCorner_Color1" |
		"forest.Hill_Top_G_OuterCorner_Color1" |
		"forest.Hill_Top_H_Side_Color1" |
		"forest.Hill_Top_I_InnerCorner_Color1" |
		"forest.Hill_Top_I_OuterCorner_Color1" |
		"forest.Rock_1_A_Color1" |
		"forest.Rock_1_B_Color1" |
		"forest.Rock_1_C_Color1" |
		"forest.Rock_1_D_Color1" |
		"forest.Rock_1_E_Color1" |
		"forest.Rock_1_F_Color1" |
		"forest.Rock_1_G_Color1" |
		"forest.Rock_1_H_Color1" |
		"forest.Rock_1_I_Color1" |
		"forest.Rock_1_J_Color1" |
		"forest.Rock_1_K_Color1" |
		"forest.Rock_1_L_Color1" |
		"forest.Rock_1_M_Color1" |
		"forest.Rock_1_N_Color1" |
		"forest.Rock_1_O_Color1" |
		"forest.Rock_1_P_Color1" |
		"forest.Rock_1_Q_Color1" |
		"forest.Rock_2_A_Color1" |
		"forest.Rock_2_B_Color1" |
		"forest.Rock_2_C_Color1" |
		"forest.Rock_2_D_Color1" |
		"forest.Rock_2_E_Color1" |
		"forest.Rock_2_F_Color1" |
		"forest.Rock_2_G_Color1" |
		"forest.Rock_2_H_Color1" |
		"forest.Rock_3_A_Color1" |
		"forest.Rock_3_B_Color1" |
		"forest.Rock_3_C_Color1" |
		"forest.Rock_3_D_Color1" |
		"forest.Rock_3_E_Color1" |
		"forest.Rock_3_F_Color1" |
		"forest.Rock_3_G_Color1" |
		"forest.Rock_3_H_Color1" |
		"forest.Rock_3_I_Color1" |
		"forest.Rock_3_J_Color1" |
		"forest.Rock_3_K_Color1" |
		"forest.Rock_3_L_Color1" |
		"forest.Rock_3_M_Color1" |
		"forest.Rock_3_N_Color1" |
		"forest.Rock_3_O_Color1" |
		"forest.Rock_3_P_Color1" |
		"forest.Rock_3_Q_Color1" |
		"forest.Rock_3_R_Color1" |
		"forest.Rock_4_A_Color1" |
		"forest.Rock_4_B_Color1" |
		"forest.Rock_4_C_Color1" |
		"forest.Rock_4_D_Color1" |
		"forest.Rock_4_E_Color1" |
		"forest.Rock_4_F_Color1" |
		"forest.Rock_4_G_Color1" |
		"forest.Rock_4_H_Color1" |
		"forest.Rock_5_A_Color1" |
		"forest.Rock_5_B_Color1" |
		"forest.Rock_5_C_Color1" |
		"forest.Rock_5_D_Color1" |
		"forest.Rock_5_E_Color1" |
		"forest.Rock_5_F_Color1" |
		"forest.Rock_5_G_Color1" |
		"forest.Rock_5_H_Color1" |
		"forest.Rock_6_A_Color1" |
		"forest.Rock_6_B_Color1" |
		"forest.Rock_6_C_Color1" |
		"forest.Rock_6_D_Color1" |
		"forest.Rock_6_E_Color1" |
		"forest.Rock_6_F_Color1" |
		"forest.Rock_6_G_Color1" |
		"forest.Rock_6_H_Color1" |
		"forest.Tree_1_A_Color1" |
		"forest.Tree_1_B_Color1" |
		"forest.Tree_1_C_Color1" |
		"forest.Tree_2_A_Color1" |
		"forest.Tree_2_B_Color1" |
		"forest.Tree_2_C_Color1" |
		"forest.Tree_2_D_Color1" |
		"forest.Tree_2_E_Color1" |
		"forest.Tree_3_A_Color1" |
		"forest.Tree_3_B_Color1" |
		"forest.Tree_3_C_Color1" |
		"forest.Tree_4_A_Color1" |
		"forest.Tree_4_B_Color1" |
		"forest.Tree_4_C_Color1" |
		"forest.Tree_5_A_Color1" |
		"forest.Tree_5_B_Color1" |
		"forest.Tree_5_C_Color1" |
		"forest.Tree_5_D_Color1" |
		"forest.Tree_5_E_Color1" |
		"forest.Tree_5_F_Color1" |
		"forest.Tree_6_A_Color1" |
		"forest.Tree_6_B_Color1" |
		"forest.Tree_6_C_Color1" |
		"forest.Tree_7_A_Color1" |
		"forest.Tree_7_B_Color1" |
		"forest.Tree_7_C_Color1" |
		"forest.Tree_Bare_1_A_Color1" |
		"forest.Tree_Bare_1_B_Color1" |
		"forest.Tree_Bare_1_C_Color1" |
		"forest.Tree_Bare_2_A_Color1" |
		"forest.Tree_Bare_2_B_Color1" |
		"forest.Tree_Bare_2_C_Color1" |
		// WEAPONS
		"weapons.arrow_A" |
		"weapons.arrow_B" |
		"weapons.arrow_C" |
		"weapons.axe_A" |
		"weapons.axe_B" |
		"weapons.axe_C" |
		"weapons.axe_D" |
		"weapons.bow_A" |
		"weapons.bow_A_withString" |
		"weapons.bow_B" |
		"weapons.bow_B_withString" |
		"weapons.bow_C" |
		"weapons.bow_C_withString" |
		"weapons.dagger_A" |
		"weapons.dagger_B" |
		"weapons.dagger_C" |
		"weapons.fistweapon_A" |
		"weapons.fistweapon_A_stacked" |
		"weapons.fistweapon_B" |
		"weapons.fistweapon_B_stacked" |
		"weapons.fistweapon_C_left" |
		"weapons.fistweapon_C_right" |
		"weapons.fistweapon_C_stacked" |
		"weapons.halberd" |
		"weapons.hammer_A" |
		"weapons.hammer_B" |
		"weapons.hammer_C" |
		"weapons.hammer_D" |
		"weapons.scythe" |
		"weapons.shield_A" |
		"weapons.shield_B" |
		"weapons.shield_C" |
		"weapons.shield_D" |
		"weapons.spear_A" |
		"weapons.spear_B" |
		"weapons.staff_A" |
		"weapons.staff_B" |
		"weapons.staff_C" |
		"weapons.staff_D" |
		"weapons.sword_A" |
		"weapons.sword_B" |
		"weapons.sword_C" |
		"weapons.sword_D" |
		"weapons.sword_E" |
		"weapons.sword_F" |
		"weapons.sword_G" |
		"weapons.wand_A" |
		"weapons.wand_B" |
        // DUNGEON
		"dungeon.banner_blue" |
		"dungeon.banner_brown" |
		"dungeon.banner_green" |
		"dungeon.banner_patternA_blue" |
		"dungeon.banner_patternA_brown" |
		"dungeon.banner_patternA_green" |
		"dungeon.banner_patternA_red" |
		"dungeon.banner_patternA_white" |
		"dungeon.banner_patternA_yellow" |
		"dungeon.banner_patternB_blue" |
		"dungeon.banner_patternB_brown" |
		"dungeon.banner_patternB_green" |
		"dungeon.banner_patternB_red" |
		"dungeon.banner_patternB_white" |
		"dungeon.banner_patternB_yellow" |
		"dungeon.banner_patternC_blue" |
		"dungeon.banner_patternC_brown" |
		"dungeon.banner_patternC_green" |
		"dungeon.banner_patternC_red" |
		"dungeon.banner_patternC_white" |
		"dungeon.banner_patternC_yellow" |
		"dungeon.banner_red" |
		"dungeon.banner_shield_blue" |
		"dungeon.banner_shield_brown" |
		"dungeon.banner_shield_green" |
		"dungeon.banner_shield_red" |
		"dungeon.banner_shield_white" |
		"dungeon.banner_shield_yellow" |
		"dungeon.banner_thin_blue" |
		"dungeon.banner_thin_brown" |
		"dungeon.banner_thin_green" |
		"dungeon.banner_thin_red" |
		"dungeon.banner_thin_white" |
		"dungeon.banner_thin_yellow" |
		"dungeon.banner_triple_blue" |
		"dungeon.banner_triple_brown" |
		"dungeon.banner_triple_green" |
		"dungeon.banner_triple_red" |
		"dungeon.banner_triple_white" |
		"dungeon.banner_triple_yellow" |
		"dungeon.banner_white" |
		"dungeon.banner_yellow" |
		"dungeon.bar_innercorner" |
		"dungeon.bar_outercorner" |
		"dungeon.bar_straight_A" |
		"dungeon.bar_straight_A_short" |
		"dungeon.bar_straight_B" |
		"dungeon.bar_straight_B_short" |
		"dungeon.bar_straight_C" |
		"dungeon.bar_straight_C_short" |
		"dungeon.barrel_large" |
		"dungeon.barrel_large_decorated" |
		"dungeon.barrel_small" |
		"dungeon.barrel_small_stack" |
		"dungeon.barrier" |
		"dungeon.barrier_column_half" |
		"dungeon.barrier_column" |
		"dungeon.barrier_corner" |
		"dungeon.barrier_half" |
		"dungeon.bartop_A_large" |
		"dungeon.bartop_A_medium" |
		"dungeon.bartop_A_medium" |
		"dungeon.bartop_B_large" |
		"dungeon.bartop_B_medium" |
		"dungeon.bartop_B_small" |
		"dungeon.bed_A_double" |
		"dungeon.bed_A_single" |
		"dungeon.bed_A_stacked" |
		"dungeon.bed_B_double" |
		"dungeon.bed_B_single" |
		"dungeon.bed_decorated" |
		"dungeon.bed_floor" |
		"dungeon.bed_floor" |
		"dungeon.bench" |
		"dungeon.book_brown" |
		"dungeon.book_grey" |
		"dungeon.book_grey" |
		"dungeon.bookcase_double" |
		"dungeon.bookcase_double" |
		"dungeon.bookcase_double" |
		"dungeon.bookcase_single" |
		"dungeon.bookcase_single" |
		"dungeon.bookcase_single" |
		"dungeon.bottle_A_brown" |
		"dungeon.bottle_A_green" |
		"dungeon.bottle_A_labeled_brown" |
		"dungeon.bottle_A_labeled_green" |
		"dungeon.bottle_B_brown" |
		"dungeon.bottle_B_green" |
		"dungeon.bottle_C_brown" |
		"dungeon.bottle_C_green" |
		"dungeon.box_large" |
		"dungeon.box_small" |
		"dungeon.box_small_decorated" |
		"dungeon.box_stacked" |
		"dungeon.bucket" |
		"dungeon.bucket_pickaxes" |
		"dungeon.candle" |
		"dungeon.candle_lit" |
		"dungeon.candle_melted" |
		"dungeon.candle_thin" |
		"dungeon.candle_thin_lit" |
		"dungeon.candle_triple" |
		"dungeon.ceiling_tile" |
		"dungeon.chair" |
		"dungeon.chest" |
		"dungeon.chest_gold" |
		"dungeon.chest_mimic_open" |
		"dungeon.coin_stack_large" |
		"dungeon.coin_stack_medium" |
		"dungeon.coin_stack_small" |
		"dungeon.column" |
		"dungeon.crate_large" |
		"dungeon.crate_large_decorated" |
		"dungeon.crate_small" |
		"dungeon.crates_stacked" |
		"dungeon.floor_dirt_large" |
		"dungeon.floor_dirt_large_rocky" |
		"dungeon.floor_dirt_small_A" |
		"dungeon.floor_dirt_small_B" |
		"dungeon.floor_dirt_small_B" |
		"dungeon.floor_dirt_small_corner" |
		"dungeon.floor_dirt_small_D" |
		"dungeon.floor_dirt_small_weeds" |
		"dungeon.floor_foundation_allsides" |
		"dungeon.floor_foundation_corner" |
		"dungeon.floor_foundation_diagonal_corner" |
		"dungeon.floor_foundation_front" |
		"dungeon.floor_foundation_front_and_back" |
		"dungeon.floor_foundation_front_and_sides" |
		"dungeon.floor_tile_big_grate" |
		"dungeon.floor_tile_big_grate_open" |
		"dungeon.floor_tile_extralarge_grates" |
		"dungeon.floor_tile_extralarge_grates_open" |
		"dungeon.floor_tile_grate" |
		"dungeon.floor_tile_grate_open" |
		"dungeon.floor_tile_large" |
		"dungeon.floor_tile_large_rocks" |
		"dungeon.floor_tile_small" |
		"dungeon.floor_tile_small_broken_A" |
		"dungeon.floor_tile_small_broken_B" |
		"dungeon.floor_tile_small_corner" |
		"dungeon.floor_tile_small_decorated" |
		"dungeon.floor_tile_small_weeds_A" |
		"dungeon.floor_tile_small_weeds_B" |
		"dungeon.floor_wood_large" |
		"dungeon.floor_wood_large_dark" |
		"dungeon.floor_wood_small" |
		"dungeon.floor_wood_small_dark" |
		"dungeon.keg" |
		"dungeon.keg_decorated" |
		"dungeon.key_gold" |
		"dungeon.keyring" |
		"dungeon.keyring_hanging" |
		"dungeon.pickaxe" |
		"dungeon.pickaxe_gold" |
		"dungeon.pillar" |
		"dungeon.pillar_decorated" |
		"dungeon.plate" |
		"dungeon.plate" |
		"dungeon.plate_food_B" |
		"dungeon.plate_small" |
		"dungeon.plate_small" |
		"dungeon.post" |
		"dungeon.rocks" |
		"dungeon.rocks_decorated" |
		"dungeon.rocks_gold" |
		"dungeon.rocks_small" |
		"dungeon.rubble_half" |
		"dungeon.rubble_large" |
		"dungeon.scaffold_beam_corner" |
		"dungeon.scaffold_beam_wall" |
		"dungeon.scaffold_beams_connected" |
		"dungeon.scaffold_frame_large" |
		"dungeon.scaffold_frame_small" |
		"dungeon.scaffold_pillar_corner" |
		"dungeon.scaffold_pillar_wall" |
		"dungeon.scaffold_pillar_wall_cross" |
		"dungeon.scaffold_pillar_wall_cross_top" |
		"dungeon.scaffold_pillar_wall_torch" |
		"dungeon.scaffold_pillars_connected" |
		"dungeon.scaffold_pillars_connected_torch" |
		"dungeon.shelf_large" |
		"dungeon.shelf_small" |
		"dungeon.shelf_small_books" |
		"dungeon.shelf_small_candles" |
		"dungeon.shelves" |
		"dungeon.shelves_decorated" |
		"dungeon.stairs" |
		"dungeon.stairs_long" |
		"dungeon.stairs_long_modular_center" |
		"dungeon.stairs_long_modular_left" |
		"dungeon.stairs_long_modular_right" |
		"dungeon.stairs_modular_center" |
		"dungeon.stairs_modular_left" |
		"dungeon.stairs_modular_right" |
		"dungeon.stairs_narrow" |
		"dungeon.stairs_wall_left" |
		"dungeon.stairs_wall_right" |
		"dungeon.stairs_walled" |
		"dungeon.stairs_wide" |
		"dungeon.stairs_wood" |
		"dungeon.stairs_wood_decorated" |
		"dungeon.stool" |
		"dungeon.stool_round" |
		"dungeon.sword_shield" |
		"dungeon.sword_shield_broken" |
		"dungeon.sword_shield_gold" |
		"dungeon.table_long" |
		"dungeon.table_long_broken" |
		"dungeon.table_long_decorated_A" |
		"dungeon.table_long_decorated_B" |
		"dungeon.table_long_decorated_C" |
		"dungeon.table_long_tablecloth" |
		"dungeon.table_long_tablecloth_decorated_A" |
		"dungeon.table_medium" |
		"dungeon.table_medium_broken" |
		"dungeon.table_medium_decorated_A" |
		"dungeon.table_medium_decorated_B" |
		"dungeon.table_medium_tablecloth" |
		"dungeon.table_medium_tablecloth_decorated_B" |
		"dungeon.table_round_large" |
		"dungeon.table_round_medium" |
		"dungeon.table_round_small" |
		"dungeon.table_small" |
		"dungeon.table_small_decorated_A" |
		"dungeon.table_small_decorated_B" |
		"dungeon.table_small_decorated_C" |
		"dungeon.torch" |
		"dungeon.torch_lit" |
		"dungeon.torch_lit" |
		"dungeon.trunk_large_A" |
		"dungeon.trunk_large_B" |
		"dungeon.trunk_large_C" |
		"dungeon.trunk_medium_A" |
		"dungeon.trunk_medium_B" |
		"dungeon.trunk_medium_C" |
		"dungeon.trunk_small_A" |
		"dungeon.trunk_small_B" |
		"dungeon.trunk_small_C" |
		"dungeon.wall" |
		"dungeon.wall_arched" |
		"dungeon.wall_archedwindow_gated" |
		"dungeon.wall_archedwindow_gated_scaffold" |
		"dungeon.wall_archedwindow_open" |
		"dungeon.wall_broken" |
		"dungeon.wall_corner" |
		"dungeon.wall_corner_gated" |
		"dungeon.wall_corner_scaffold" |
		"dungeon.wall_corner_small" |
		"dungeon.wall_cracked" |
		"dungeon.wall_crossing" |
		"dungeon.wall_doorway_sides" |
		"dungeon.wall_doorway_Tsplit" |
		"dungeon.wall_endcap" |
		"dungeon.wall_gated" |
		"dungeon.wall_half" |
		"dungeon.wall_half_endcap" |
		"dungeon.wall_half_endcap_sloped" |
		"dungeon.wall_inset" |
		"dungeon.wall_inset_candles" |
		"dungeon.wall_inset_shelves" |
		"dungeon.wall_inset_shelves_broken" |
		"dungeon.wall_inset_shelves_decoratedA" |
		"dungeon.wall_inset_shelves_decoratedB" |
		"dungeon.wall_open_scaffold" |
		"dungeon.wall_pillar" |
		"dungeon.wall_scaffold" |
		"dungeon.wall_shelves" |
		"dungeon.wall_sloped" |
		"dungeon.wall_Tsplit" |
		"dungeon.wall_Tsplit_sloped" |
		"dungeon.wall_window_closed" |
		"dungeon.wall_window_closed_scaffold" |
		"dungeon.wall_window_open" |
		"dungeon.wall_window_open_scaffold" |
		"dungeon.chest_empty" |
		"dungeon.chest_empty_bottom" |
		"dungeon.chest_lid" |
		"dungeon.chest_gold" |
		"dungeon.chest_large_closed" |
		"dungeon.chest_large_empty_open" |
		"dungeon.chest_large_empty_bottom" |
		"dungeon.chest_large_empty_lid" |
		"dungeon.chest_large_gold_closed" |
		"dungeon.chest_large_gold_rigged" |
		"dungeon.floor_tile_big_spikes_empty" |
		"dungeon.floor_tile_big_spikes_poking" |
		"dungeon.floor_tile_big_spikes_out" |
		"dungeon.wall_doorway_empty" |
		"dungeon.wall_doorway_scaffold_empty" |
		"dungeon.wall_doorway_closed" |
		"dungeon.wall_doorway_scaffold_closed" |
		"dungeon.wall_doorway_scaffold_open" |
		"dungeon.wall_doorway_closed" |
		"dungeon.wall_door" |
		"dungeon.bartop_A_small" |
		"dungeon.bed_frame" |
		"dungeon.book_tan" |
		"dungeon.bookcase_double_decoratedA" |
		"dungeon.bookcase_double_decoratedB" |
		"dungeon.bookcase_single_decoratedA" |
		"dungeon.bookcase_single_decoratedB" |
		"dungeon.floor_dirt_small_C" |
		"dungeon.plate_food_A" |
		"dungeon.torch_mounted" |
		"dungeon.chest_large_gold_open" |
		"dungeon.wall_doorway_open" |
        // CITY
		"city.base" |
		"city.bench" |
		"city.box_A" |
		"city.box_B" |
		"city.building_A" |
		"city.building_A_withoutBase" |
		"city.building_B" |
		"city.building_B_withoutBase" |
		"city.building_C" |
		"city.building_C_withoutBase" |
		"city.building_D" |
		"city.building_D_withoutBase" |
		"city.building_E" |
		"city.building_E_withoutBase" |
		"city.building_F" |
		"city.building_F_withoutBase" |
		"city.building_G" |
		"city.building_G_withoutBase" |
		"city.building_H" |
		"city.building_H_withoutBase" |
		"city.bush" |
		"city.bush_A" |
		"city.bush_B" |
		"city.bush_C" |
		"city.car_hatchback" |
		"city.car_police" |
		"city.car_sedan" |
		"city.car_stationwagon" |
		"city.car_taxi" |
		"city.dumpster" |
		"city.firehydrant" |
		"city.park_base" |
		"city.park_base_decorated_bushes" |
		"city.park_base_decorated_trees" |
		"city.park_road_corner" |
		"city.park_road_corner_decorated" |
		"city.park_road_junction" |
		"city.park_road_junction_decorated_A" |
		"city.park_road_junction_decorated_B" |
		"city.park_road_junction_decorated_C" |
		"city.park_road_straight" |
		"city.park_road_straight_decorated_A" |
		"city.park_road_straight_decorated_B" |
		"city.park_road_tsplit" |
		"city.park_road_tsplit_decorated" |
		"city.park_wall_entry" |
		"city.park_wall_entry_decorated" |
		"city.park_wall_innerCorner" |
		"city.park_wall_innerCorner_decorated" |
		"city.park_wall_outerCorner" |
		"city.park_wall_outerCorner_decorated" |
		"city.park_wall_straight" |
		"city.park_wall_straight_decorated" |
		"city.road_corner" |
		"city.road_corner_curved" |
		"city.road_junction" |
		"city.road_straight" |
		"city.road_straight_crossing" |
		"city.road_tsplit" |
		"city.streetlight" |
		"city.streetlight_old_double" |
		"city.streetlight_old_single" |
		"city.trafficlight_A" |
		"city.trafficlight_B" |
		"city.trafficlight_C" |
		"city.trash_A" |
		"city.trash_B" |
		"city.tree_A" |
		"city.tree_B" |
		"city.tree_C" |
		"city.tree_D" |
		"city.tree_E" |
		"city.watertower" |
        // BLOCK
		"block.anvil" |
		"block.apple" |
		"block.barrel" |
		"block.battery" |
		"block.gift" |
		"block.hay" |
		"block.stone" |
		"block.stone_dark" |
		"block.trashcan" |
		"block.water" |
		"block.books_A" |
		"block.books_B" |
		"block.bricks_A" |
		"block.bricks_B" |
		"block.chest" |
		"blocks.colored_block_blue" |
		"block.colored_block_green" |
		"block.colored_block_red" |
		"block.colored_block_yellow" |
		"block.crate" |
		"block.decorative_block_blue" |
		"block.decorative_block_green" |
		"block.decorative_block_red" |
		"block.decorative_block_yellow" |
		"block.dirt" |
		"block.dirt_with_grass" |
		"block.dirt_with_snow" |
		"blocks.dynamite" |
		"block.glass" |
		"block.grass" |
		"block.grass_with_snow" |
		"blocks.gravel" |
		"block.gravel_with_grass" |
		"block.gravel_with_snow" |
		"block.hay_bale" |
		"block.lava" |
		"block.melon" |
		"block.metal" |
		"block.metalframe" |
		"block.pipe" |
		"block.prototype" |
		"block.sand_A" |
		"block.sand_B" |
		"block.sand_with_grass" |
		"block.sand_with_snow" |
		"block.snow" |
		"blocks.stone_with_copper" |
		"block.stone_with_gold" |
		"block.stone_with_silver" |
		"block.striped_block_blue" |
		"block.striped_block_green" |
		"block.striped_block_red" |
		"block.striped_block_yellow" |
		"block.tree" |
		"block.tree_with_snow" |
		"block.vault" |
		"block.wood" |
		"blocks.computer" | "computer" => Some(Instance::DComputer(Computer::new(entt))),
        "floor" => Some(Instance::DFloor(Floor::new(entt))),
        "viridian_house" => Some(Instance::DFloor(Floor::new(entt))),
        "viridian_floor" => Some(Instance::DFloor(Floor::new(entt))),
        "tree" => Some(Instance::DFloor(Floor::new(entt))),
        "platform" => Some(Instance::DPlatform(Platform::new(entt))),
        "table" => Some(Instance::DTable(Table::new(entt))),
        // entities
        "barrier" => Some(Instance::EBarrier(Barrier::new(entt))),
        "gcyl" => Some(Instance::EGcyl(Gcyl::new(entt))),
        "light" => Some(Instance::ELight(Light::new(entt))),
        "player" => Some(Instance::EPlayer(Player::new(entt))),
        "pig" => Some(Instance::EPig(Pig::new(entt))),
        "menu_m" => Some(Instance::EMenuM(Menu::new(entt))),
        "menu_e" => Some(Instance::EMenuE(Menu::new(entt))),
        "menu_n" => Some(Instance::EMenuN(Menu::new(entt))),
        "menu_u" => Some(Instance::EMenuU(Menu::new(entt))),
        "trigger_levelchange" => None,
        unknown => {
            eprintln!("unrecognized entity '{}'", unknown);
            None
        }
    }
}

impl Instance {
    pub fn update(&mut self) {
        match self {
            Self::DComputer(e) => e.update(),
            Self::DFloor(e) => e.update(),
            Self::DPlatform(e) => e.update(),
            Self::DTable(e) => e.update(),
            Self::EBarrier(e) => e.update(),
            Self::EGcyl(e) => e.update(),
            Self::ELight(e) => e.update(),
            Self::EPlayer(e) => e.update(),
            Self::EPig(e) => e.update(),
            Self::EMenuM(e) => e.update(),
            Self::EMenuE(e) => e.update(),
            Self::EMenuN(e) => e.update(),
            Self::EMenuU(e) => e.update(),
            Self::ETriggerLevelChange => {
                panic!("unimplemented")
            }
        }
    }

    pub fn draw_model(&mut self) {
        match self {
            Self::DComputer(e) => e.draw_model(),
            Self::DFloor(e) => e.draw_model(),
            Self::DPlatform(e) => e.draw_model(),
            Self::DTable(e) => e.draw_model(),
            Self::EBarrier(e) => e.draw_model(),
            Self::EGcyl(e) => e.draw_model(),
            Self::ELight(e) => e.draw_model(),
            Self::EPlayer(e) => e.draw_model(),
            Self::EPig(e) => e.draw_model(),
            Self::EMenuM(e) => e.draw_model(),
            Self::EMenuE(e) => e.draw_model(),
            Self::EMenuN(e) => e.draw_model(),
            Self::EMenuU(e) => e.draw_model(),
            Self::ETriggerLevelChange => {
                panic!("unimplemented")
            }
        }
    }

    pub fn is_decor(&mut self) -> bool {
        match self {
            Self::DComputer(_) => true,
            Self::DFloor(_) => true,
            Self::DPlatform(_) => true,
            Self::DTable(_) => true,
            Self::EBarrier(_) => false,
            Self::EGcyl(_) => false,
            Self::ELight(_) => false,
            Self::EPlayer(_) => false,
            Self::EPig(_) => false,
            Self::EMenuM(_) => false,
            Self::EMenuE(_) => false,
            Self::EMenuN(_) => false,
            Self::EMenuU(_) => false,
            Self::ETriggerLevelChange => {
                panic!("unimplemented")
            }
        }
    }

    pub fn get_mesh(&mut self) -> Vec<[raymath::Vector3; 3]> {
        match self {
            // get meshes for decor
            Self::DComputer(e) => e.get_mesh(),
            Self::DFloor(e) => e.get_mesh(),
            Self::DPlatform(e) => e.get_mesh(),
            Self::DTable(e) => e.get_mesh(),
            // rest will panic
            Self::EBarrier(e) => e.get_mesh(),
            Self::EGcyl(e) => e.get_mesh(),
            Self::ELight(e) => e.get_mesh(),
            Self::EPlayer(e) => e.get_mesh(),
            Self::EPig(e) => e.get_mesh(),
            Self::EMenuM(e) => e.get_mesh(),
            Self::EMenuE(e) => e.get_mesh(),
            Self::EMenuN(e) => e.get_mesh(),
            Self::EMenuU(e) => e.get_mesh(),
            Self::ETriggerLevelChange => {
                panic!("unimplemented")
            }
        }
    }

    pub fn get_matrix(&mut self) -> raymath::Matrix {
        match self {
            // get mat for decor
            Self::DComputer(e) => e.get_matrix(),
            Self::DFloor(e) => e.get_matrix(),
            Self::DPlatform(e) => e.get_matrix(),
            Self::DTable(e) => e.get_matrix(),
            // rest will panic
            Self::EBarrier(e) => e.get_matrix(),
            Self::EGcyl(e) => e.get_matrix(),
            Self::ELight(e) => e.get_matrix(),
            Self::EPlayer(e) => e.get_matrix(),
            Self::EPig(e) => e.get_matrix(),
            Self::EMenuM(e) => e.get_matrix(),
            Self::EMenuE(e) => e.get_matrix(),
            Self::EMenuN(e) => e.get_matrix(),
            Self::EMenuU(e) => e.get_matrix(),
            Self::ETriggerLevelChange => {
                panic!("unimplemented")
            }
        }
    }
}

// marking as deprecated because it's slower than hell
#[deprecated]
pub fn pos_is_visible(cam_pos: Vector3, point: Vector3) -> bool {
    let decs = get_decor_instances().unwrap();
    let dir = vector3_normalize(vector3_subtract(point, cam_pos));
    let distance = vector3_distance(cam_pos, point);
    let ray = raymath::Ray {
        position: cam_pos,
        direction: dir,
    };

    // find nearest decor collision
    for dec in decs {
        let dec = &mut *dec;
        let mesh = dec.get_mesh();
        let mat = dec.get_matrix();

        // max distance hack. exclude
        if vector3_distance(cam_pos, point) > 64. {
            return false;
        }

        let coll = math::get_ray_collision_mesh(ray, mesh, mat, None);
        // collides before reaching point
        if coll.hit && coll.distance < distance {
            return false;
        }
    }

    true
}

pub fn get_decor_instances<'a>() -> Result<Vec<&'a mut Instance>, NUError> {
    g_game::get_filtered_instances(|inst| inst.is_decor())
}

pub fn get_barrier_instances<'a>() -> Result<Vec<&'a mut Instance>, NUError> {
    g_game::get_filtered_instances(|inst| match inst {
        Instance::EBarrier(_) => true,
        _ => false,
    })
}

pub fn get_player_instance<'a>() -> Result<&'a mut Player, NUError> {
    let mut insts = g_game::get_filtered_instances(|inst| match inst {
        Instance::EPlayer(_) => true,
        _ => false,
    })?;

    if insts.len() != 1 {
        return Err(NUError::MiscError("expected exactly one player".into()));
    }
    
    let player_inst = insts.remove(0);
    match player_inst {
        Instance::EPlayer(player) => Ok(player),
        _ => Err(NUError::MiscError("expected exactly one player".into()))
    }
}
