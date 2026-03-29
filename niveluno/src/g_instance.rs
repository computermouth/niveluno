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
