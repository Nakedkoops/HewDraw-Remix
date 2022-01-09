use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn dim_cape_early_attack_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if frame > 10.0 {
            if hdr::compare_cat(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, false);
            }
        }
    }
}

// Meta Knight Special Fall Hit Reset
// Set flags for each special move
unsafe fn flag_resets(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, motion_kind: u64, frame: f32) {
    if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN {
            neutral_special_hit[id] = true;
        } else if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH {
            side_special_hit[id] = true;
        } else if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP].contains(&status_kind) {
            up_special_hit[id] = true;
        } else if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK {
            down_special_hit[id] = true;
        }
    }
}

// Transition to fall
unsafe fn transition_fall(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        let prev_status = StatusModule::prev_status_kind(boma, 0);
        if (prev_status == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END && neutral_special_hit[id])
            || (prev_status == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END && side_special_hit[id])
            || (prev_status == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP && up_special_hit[id])
            || ([*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK].contains(&prev_status) && down_special_hit[id]) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

// Reset flags
unsafe fn reset_flags(id: usize, status_kind: i32, situation_kind: i32) {
    if situation_kind != SITUATION_KIND_AIR
        || [*FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_WIN,
            *FIGHTER_STATUS_KIND_LOSE,
            *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
            neutral_special_hit[id] = false;
            side_special_hit[id] = false;
            up_special_hit[id] = false;
            down_special_hit[id] = false;
    }
}

// Lengthen sword
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
	let long_sword_scale = Vector3f{x: 1.0, y: 1.15, z: 1.1};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &long_sword_scale);
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
}				 
pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    dim_cape_early_attack_cancel(boma, status_kind, frame);
    flag_resets(boma, id, status_kind, motion_kind, frame);
    transition_fall(boma, id, status_kind);
    reset_flags(id, status_kind, situation_kind);
    sword_length(boma);
}

#[utils::opff(FIGHTER_KIND_METAKNIGHT )]
pub fn metaknight_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		metaknight_frame(fighter)
    }
}

pub unsafe fn metaknight_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}