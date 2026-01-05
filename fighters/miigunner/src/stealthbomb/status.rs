use super::*;

// todo move hardcoded param values up here as consts

unsafe extern "C" fn move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = weapon.get_param_float("param_stealthbomb", "life");
    weapon.set_float(life, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE);
    weapon.set_float(0.0, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_COUNT);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        move_substatus(weapon, false.into());
    }
    weapon.global_table[0x15].assign(&L2CValue::Ptr(move_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(move_main_loop as *const () as _))
}

unsafe extern "C" fn move_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        //println!("sub move");
        //println!();
        WorkModule::add_float(weapon.module_accessor, -1.0, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE);
        WorkModule::add_float(weapon.module_accessor, 1.0, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_COUNT);
        let count = weapon.get_param_float("param_stealthbomb", "count");
        if weapon.get_float(*WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_COUNT) > count {
            weapon.set_float(count, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_COUNT);
        }
    }

    return 0.into();
}

unsafe extern "C" fn move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.get_float(*WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE) <= 0.0 {
        weapon.change_status(WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_KIND_TAME.into(), false.into());
    }
    // removed GroundModule check

    return 0.into();
}

unsafe extern "C" fn move_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let angle = weapon.get_float(*WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_ANGLE);
    VarModule::set_float(weapon.battle_object, vars::miigunner_stealthbomb::instance::ANGLE, angle);
    //println!("angle: {}", angle);
    //println!();

    return 0.into();
}

unsafe extern "C" fn tame_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let bang_time = 60.0;   //weapon.get_param_float("param_stealthbomb", "bang_time");
    weapon.set_float(bang_time, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE);
    let angle = VarModule::get_float(weapon.battle_object, vars::miigunner_stealthbomb::instance::ANGLE).to_degrees(); //weapon.get_float(*WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_ANGLE);
    println!("angle: {}", angle);
    println!("angle_cos: {}", angle.cos());
    println!("angle_sin: {}", angle.sin());
    println!();
    //sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 1.0 * angle.cos(), 1.0 * angle.sin());
    return 0.into();
}

unsafe extern "C" fn tame_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn tame_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("tame"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        tame_substatus(weapon, false.into());
    }
    weapon.global_table[0x15].assign(&L2CValue::Ptr(tame_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(tame_main_loop as *const () as _))
}

unsafe extern "C" fn tame_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        //println!("sub tame true");
        //println!();
        WorkModule::add_float(weapon.module_accessor, -1.0, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE);
    }
    else {
        //println!("sub tame false");
        if weapon.get_float(*WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE) <= 0.0 {
            //println!("life <= 0.0");
            //println!("time to turn");
            // weapon.clear_lua_stack();
            // lua_args!(weapon, MA_MSC_CMD_ARTICLE_GENERATE_ARTICLE_LINK_PARENTS, WEAPON_LINK_NO_CONSTRAINT, FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB_S);
            // sv_module_access::article(weapon.lua_state_agent);
            // weapon.pop_lua_stack(1);
            // notify_event_msc_cmd!(weapon, Hash40::new_raw(0x27936dbb96d));

            weapon.change_status(statuses::miigunner_stealthbomb::TURN.into(), false.into());
        }
        //println!();
    }

    return 0.into();
}

unsafe extern "C" fn tame_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn turn_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    println!("TURN_PRE");
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0,
    );

    return 0.into();
}

unsafe extern "C" fn turn_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    println!("TURN_INIT");
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);

    //let angle = WorkModule::get_float(weapon.module_accessor, WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_ANGLE);    //*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let angle = VarModule::get_float(weapon.battle_object, vars::miigunner_stealthbomb::instance::ANGLE);
    println!("angle (turn init): {}", angle);
    let new_angle = if angle <= 0.0 {
        angle + std::f32::consts::PI
    } else {
        angle - std::f32::consts::PI
    };
    println!("new angle: {}", new_angle);
    //WorkModule::set_float(weapon.module_accessor, new_angle, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
    VarModule::set_float(weapon.battle_object, vars::miigunner_stealthbomb::instance::ANGLE, new_angle);

    let follow_frame = 128; //WorkModule::get_param_int(weapon.module_accessor, hash40("param_boomerang"), hash40("follow_frame"));
    //WorkModule::set_int(weapon.module_accessor, follow_frame, *WN_LINK_BOOMERANG_TURN_WORK_INT_FOLLOW_FRAME);
    VarModule::set_int(weapon.battle_object, vars::miigunner_stealthbomb::status::FOLLOW_FRAME, follow_frame);
    
    //add
    WorkModule::set_float(weapon.module_accessor, follow_frame as f32, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE);

    let rot_x = PostureModule::rot_x(weapon.module_accessor, 0);//*WN_LINK_BOOMERANG_POSTURE_ROT_NODE_TOPN);
    let rot_y = PostureModule::rot_y(weapon.module_accessor, 0);//*WN_LINK_BOOMERANG_POSTURE_ROT_NODE_TOPN);
    let rot_z = PostureModule::rot_z(weapon.module_accessor, 0);//*WN_LINK_BOOMERANG_POSTURE_ROT_NODE_TOPN);

    let angle_x_turn = -16.0; //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("angle_x_turn"));

    //PostureModule::set_rot(weapon.module_accessor, &Vector3f{ x: rot_x, y: angle_x_turn, z: rot_z }, *WN_LINK_BOOMERANG_POSTURE_ROT_NODE_TOPN);

    //WorkModule::set_float(weapon.module_accessor, 0.0, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_TURN_DIST);
    VarModule::set_float(weapon.battle_object, vars::miigunner_stealthbomb::status::TURN_DIST, 0.0);

    let rot_speed = 22.0;   //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("rot_speed"));
    let angle_x_back = -16.0;   //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("angle_x_back"));
    let speed = WorkModule::get_float(weapon.module_accessor, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_SPEED);   //*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_SPEED);
    let accel = 0.047;  //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("accel"));
    let speed_min = 0.3;    //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_min"));
    let speed_diff = speed - speed_min;
    let accel_diff = speed_diff / accel;
    let floor = accel_diff.floor();
    let idkman = angle_x_back - rot_y;
    let huh = idkman / floor;
    //WorkModule::set_int(weapon.module_accessor, floor as i32, *WN_LINK_BOOMERANG_TURN_WORK_INT_BACK_ROT_FRAME);
    VarModule::set_int(weapon.battle_object, vars::miigunner_stealthbomb::status::BACK_ROT_FRAME, floor as i32);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, rot_speed, huh, 0.0);

    return 0.into();
}

unsafe extern "C" fn turn_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    println!("TURN_MAIN");
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("turn"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        if false {
            turn_substatus_inner(weapon);
        }
    }
    weapon.global_table[0x15].assign(&L2CValue::Ptr(turn_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(turn_fastshift as *const () as _))
}

unsafe extern "C" fn turn_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        turn_substatus_inner(weapon);
    }

    return 0.into();
}

unsafe extern "C" fn turn_substatus_inner(weapon: &mut L2CWeaponCommon) {
    //println!("turn_substatus_inner");
    //if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE, 0) {    //*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_LIFE, 0) {
    let follow_frame = VarModule::get_int(weapon.battle_object, vars::miigunner_stealthbomb::status::FOLLOW_FRAME);
    println!("follow_frame: {}", follow_frame);
    if VarModule::get_int(weapon.battle_object, vars::miigunner_stealthbomb::status::FOLLOW_FRAME) == 0 {
        println!("life countdown 0");
        println!("time to die");
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn turn_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    println!("hit check");
    println!();
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
        println!("imgonnablowup");
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_CMD_ARTICLE_GENERATE_ARTICLE_LINK_PARENTS, WEAPON_LINK_NO_CONSTRAINT, FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB_S);
        sv_module_access::article(weapon.lua_state_agent);
        weapon.pop_lua_stack(1);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x27936dbb96d));
    }
    if !StopModule::is_stop(weapon.module_accessor) {
        if turn_fastshift_inner(weapon).get_bool() {
            return 1.into();
        }
    }

    return 0.into();
}

unsafe extern "C" fn turn_fastshift_inner(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let correct = GroundModule::get_correct(weapon.module_accessor);
    if LinkModule::is_link(weapon.module_accessor, *LINK_NO_ARTICLE) {
        let parent_id = LinkModule::get_parent_id(weapon.module_accessor, *LINK_NO_ARTICLE, true);
        if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLAG_REFLECT) {   //*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_REFLECT) {
        //if !VarModule::is_flag(weapon.battle_object, vars::miigunner_stealthbomb::status::REFLECT) {
            let team_owner_id = TeamModule::team_owner_id(weapon.module_accessor);
            if parent_id == team_owner_id {
                weapon.clear_lua_stack();
                lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_X, LINK_NO_ARTICLE, Hash40::new("waist"), true);
                FL_sv_module_access::link(weapon.lua_state_agent);
                let x = weapon.pop_lua_stack(1).get_f32();
                weapon.clear_lua_stack();
                lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Y, LINK_NO_ARTICLE, Hash40::new("waist"), true);
                FL_sv_module_access::link(weapon.lua_state_agent);
                let y = weapon.pop_lua_stack(1).get_f32();
                weapon.clear_lua_stack();
                lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Z, LINK_NO_ARTICLE, Hash40::new("waist"), true);
                FL_sv_module_access::link(weapon.lua_state_agent);
                let z = weapon.pop_lua_stack(1).get_f32();

                let pos_x = PostureModule::pos_x(weapon.module_accessor);
                let pos_y = PostureModule::pos_y(weapon.module_accessor);
                let pos_z = PostureModule::pos_z(weapon.module_accessor);

                // println!("----------------");
                // println!("poz_x: {}", pos_x);
                // println!("poz_y: {}", pos_y);
                // println!("poz_z: {}", pos_z);
                // println!("x: {}", x);
                // println!("y: {}", y);
                // println!("z: {}", z);
                // println!("----------------");

                let length = sv_math::vec3_length(x - pos_x, y - pos_y, z - pos_z);
                if length <= 9.0 {
                    //weapon.clear_lua_stack();
                    //lua_args!(weapon, MA_MSC_LINK_SEND_EVENT_PARENTS, LINK_NO_ARTICLE, Hash40::new_raw(0x170db96f9c), WN_LINK_BOOMERANG_TURN_WORK_INT_LINK_EVENT_RESULT_01, WN_LINK_BOOMERANG_TURN_WORK_FLOAT_LINK_EVENT_RESULT_01, WN_LINK_BOOMERANG_TURN_WORK_FLAG_LINK_EVENT_RESULT_01);
                    //sv_module_access::link(weapon.lua_state_agent);
                    //if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_TURN_WORK_INT_LINK_EVENT_RESULT_01) {
                        //PostureModule::set_rot(weapon.module_accessor, &Vector3f{ x: 0.0, y: 0.0, z: 0.0 }, *WN_LINK_BOOMERANG_POSTURE_ROT_NODE_TOPN);
                        //PostureModule::set_rot(weapon.module_accessor, &Vector3f{ x: 0.0, y: 0.0, z: 0.0 }, *WN_LINK_BOOMERANG_POSTURE_ROT_NODE_ROTN);
                        //WorkModule::set_float(weapon.module_accessor, 0.0, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
                        //weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_HAVED.into(), false.into());
                        //return 1.into();
                    //}
                    //else {
                    //    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
                    //    return 1.into();
                    //}
                    println!("length <= 9.0");
                    println!("commit death");
                    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
                    return 1.into();
                }
            }
        }

        if correct != *GROUND_CORRECT_KIND_NONE {
            let parent_id = LinkModule::get_parent_id(weapon.module_accessor, *LINK_NO_ARTICLE, true);
            let parent_module_accessor = sv_battle_object::module_accessor(parent_id as u32);
            let parent_pos_y = PostureModule::pos_y(parent_module_accessor);
            let pos_y = PostureModule::pos_y(weapon.module_accessor);
            let diff = parent_pos_y - pos_y;
            let dist = 4.0; //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), 0x1cb25d4dc6);
            if diff.abs() >= dist * 10.0 {
                GroundModule::set_correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
            }
        }
    }

    let correct = GroundModule::get_correct(weapon.module_accessor);

    let turn_dist = 16.0;   //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("turn_dist"));
    //let dist = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_TURN_DIST);
    let dist = VarModule::get_float(weapon.battle_object, vars::miigunner_stealthbomb::status::TURN_DIST);
    if turn_dist * 10.0 <= dist {
        println!("turn_dist <= dist");
        println!("perish");
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 1.into();
    }

    // if correct == *GROUND_CORRECT_KIND_NONE {
    //     if !StatusModule::is_changing(weapon.module_accessor) {
    //         if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
    //             GroundModule::set_correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
    //             return 0.into();
    //         }
    //     }
    //     if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_UP | *GROUND_TOUCH_FLAG_DOWN) as u32) {
    //         notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
    //     }
    // }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x27936dbb96d));
    }

    //add
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_PARRY | *COLLISION_KIND_MASK_REFLECTOR) {
        VarModule::on_flag(weapon.battle_object, vars::miigunner_stealthbomb::status::REFLECT);
    }
    // if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
    //     println!("imgonnablowup");
    //     weapon.clear_lua_stack();
    //     lua_args!(weapon, MA_MSC_CMD_ARTICLE_GENERATE_ARTICLE_LINK_PARENTS, WEAPON_LINK_NO_CONSTRAINT, FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB_S);
    //     sv_module_access::article(weapon.lua_state_agent);
    //     weapon.pop_lua_stack(1);
    //     notify_event_msc_cmd!(weapon, Hash40::new_raw(0x27936dbb96d));
    // }

    return 0.into();
}

unsafe extern "C" fn turn_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    turn_exec_inner(weapon)
}

unsafe extern "C" fn turn_exec_inner(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //let mut angle = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let mut angle = VarModule::get_float(weapon.battle_object, vars::miigunner_stealthbomb::instance::ANGLE);
    if LinkModule::is_link(weapon.module_accessor, *LINK_NO_ARTICLE) {
        if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLAG_REFLECT) //*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_REFLECT)
        //&& WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_TURN_WORK_INT_FOLLOW_FRAME) > 0 {
        //if !VarModule::is_flag(weapon.battle_object, vars::miigunner_stealthbomb::status::REFLECT)
        && VarModule::get_int(weapon.battle_object, vars::miigunner_stealthbomb::status::FOLLOW_FRAME) > 0 {
            weapon.clear_lua_stack();
            lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_X, LINK_NO_ARTICLE, Hash40::new("waist"), true);
            FL_sv_module_access::link(weapon.lua_state_agent);
            let x = weapon.pop_lua_stack(1).get_f32();
            weapon.clear_lua_stack();
            lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Y, LINK_NO_ARTICLE, Hash40::new("waist"), true);
            FL_sv_module_access::link(weapon.lua_state_agent);
            let y = weapon.pop_lua_stack(1).get_f32();
            weapon.clear_lua_stack();

            let pos_x = PostureModule::pos_x(weapon.module_accessor);
            let pos_y = PostureModule::pos_y(weapon.module_accessor);
            
            let diff_x = x - pos_x;
            let diff_y = y - pos_y;

            println!("diff_x: {}", diff_x);
            println!("diff_y: {}", diff_y);
            
            let atan = diff_y.atan2(diff_x);

            println!("atan: {}", atan);
            
            let atan = if atan < -std::f32::consts::PI {
                atan + std::f32::consts::PI * 2.0
            }
            else {
                if std::f32::consts::PI < atan {
                    atan - std::f32::consts::PI * 2.0
                }
                else {
                    atan
                }
            };

            let atan = atan - angle;

            println!("angle: {}", angle);
            println!("corrected atan: {}", atan);
            
            let atan = if atan < -std::f32::consts::PI {
                atan + std::f32::consts::PI * 2.0
            }
            else {
                if std::f32::consts::PI < atan {
                    atan - std::f32::consts::PI * 2.0
                }
                else {
                    atan
                }
            };

            println!("re-corrected atan: {}", atan);

            let turn_angle = 0.75;  //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("turn_angle")).to_radians();
            let atan = if turn_angle < atan {
                turn_angle
            }
            else {
                if atan < -turn_angle {
                    -turn_angle
                }
                else {
                    atan
                }
            };

            println!("final atan: {}", atan);
            angle = atan;

            //WorkModule::set_float(weapon.module_accessor, atan, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
            VarModule::set_float(weapon.battle_object, vars::miigunner_stealthbomb::instance::ANGLE, atan);
            //WorkModule::dec_int(weapon.module_accessor, *WN_LINK_BOOMERANG_TURN_WORK_INT_FOLLOW_FRAME);
            VarModule::dec_int(weapon.battle_object, vars::miigunner_stealthbomb::status::FOLLOW_FRAME);
        }
    }

    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let mut length = sv_kinetic_energy::get_speed_length(weapon.lua_state_agent);
    let accel = 0.047;  //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("accel"));
    length += accel;
    let speed_max = 3.0;    //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_max"));
    let speed_mul = 1.0;    //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_mul"));
    let speed_max = speed_max * speed_mul;
    if speed_max < length {
        length = speed_max;
    }
    let cos = angle.cos();
    let sin = angle.sin();
    let vel_x = cos * length;
    let vel_y = sin * length;
    println!("setting speed!");
    println!("vel_x: {}", vel_x);
    println!("vel_y: {}", vel_y);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, vel_x, vel_y);

    //let turn_dist = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_TURN_DIST);
    let turn_dist = VarModule::get_float(weapon.battle_object, vars::miigunner_stealthbomb::status::TURN_DIST);
    //WorkModule::set_float(weapon.module_accessor, turn_dist + length, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_TURN_DIST);
    VarModule::set_float(weapon.battle_object, vars::miigunner_stealthbomb::status::TURN_DIST, turn_dist + length);

    let turn_follow_dist = 19.0;    //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("turn_follow_dist"));
    if turn_follow_dist * 10.0 <= turn_dist + length {
        println!("turn_follow_dist <= turn_dist + length");
        println!("guess I'll die");
        //WorkModule::set_int(weapon.module_accessor, 0, *WN_LINK_BOOMERANG_TURN_WORK_INT_FOLLOW_FRAME);
        VarModule::set_int(weapon.battle_object, vars::miigunner_stealthbomb::status::FOLLOW_FRAME, 0);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }

    //let back_rot_frame = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_TURN_WORK_INT_BACK_ROT_FRAME);
    let back_rot_frame = VarModule::get_int(weapon.battle_object, vars::miigunner_stealthbomb::status::BACK_ROT_FRAME);
    if back_rot_frame > 0 {
        if back_rot_frame - 1 == 0 {
            let rot_speed = 22.0;   //WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("rot_speed"));
            sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL, rot_speed, 0.0, 0.0);
        }
        //WorkModule::dec_int(weapon.module_accessor, *WN_LINK_BOOMERANG_TURN_WORK_INT_BACK_ROT_FRAME);
        VarModule::dec_int(weapon.battle_object, vars::miigunner_stealthbomb::status::BACK_ROT_FRAME);
    }
    println!();

    return 0.into();
}

unsafe extern "C" fn turn_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    println!("end");
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_CMD_ARTICLE_GENERATE_ARTICLE_LINK_PARENTS, WEAPON_LINK_NO_CONSTRAINT, FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB_S);
        sv_module_access::article(weapon.lua_state_agent);
        weapon.pop_lua_stack(1);
    }
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_KIND_MOVE, move_main);
    agent.status(End, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_KIND_MOVE, move_end);

    agent.status(Init, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_KIND_TAME, tame_init);
    agent.status(Main, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_KIND_TAME, tame_main);

    agent.status(Pre, statuses::miigunner_stealthbomb::TURN, turn_pre);
    agent.status(Init, statuses::miigunner_stealthbomb::TURN, turn_init);
    agent.status(Main, statuses::miigunner_stealthbomb::TURN, turn_main);
    agent.status(Exec, statuses::miigunner_stealthbomb::TURN, turn_exec);
    agent.status(End, statuses::miigunner_stealthbomb::TURN, turn_end);
}