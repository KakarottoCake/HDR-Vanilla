use super::*;
use utils::ext::*;
use std::arch::asm;

const NUM_ANGLE_CHECK: i32 = 12;

extern "C" {
    #[link_name = "_ZN3app6camera13get_dead_areaEv"]
    fn get_dead_area() -> Rect;
}

#[repr(simd)]
#[derive(Debug)]
struct Rect {
    // left: f32,
    // right: f32,
    // top: f32,
    // bottom: f32,
    vec: [f32; 4]
}

impl Rect {
    fn contains(&self, x: f32, y: f32) -> bool {
        (self.vec[0] <= x && x <= self.vec[1]) && (self.vec[3] <= y && y <= self.vec[2])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct KnockbackCalcContext {
    pub defender_boma: *mut BattleObjectModuleAccessor,
    pub fly_top_angle_lw: f32,
    pub fly_top_angle_hi: f32,
    pub ecb_bottom: Vector4f,
    pub ecb_left: Vector4f,
    pub ecb_right: Vector4f,

    pub knockback: f32,
    pub hitstun: f32,
    pub damage: f32,
    pub sdi_mul: f32,
    pub launch_radians: f32,
    pub launch_speed: Vector2f,
    pub y_chara_speed: f32,
    pub is_tumble: bool,
    pub is_damage_fly_top: bool,
    pub gravity: f32,
    pub fall_speed: f32,
    pub pos: Vector2f,
    pub pos_prev: Vector2f,
    pub damage_air_brake: f32,
    pub speed_up_mul: f32,

    pub is_tech_possible: bool,
}

impl KnockbackCalcContext {
    pub unsafe fn new(
        defender_boma: *mut BattleObjectModuleAccessor,
        knockback: f32,
        hitstun: f32,
        damage: f32,
        sdi_mul: f32,
        launch_radians: f32,
        launch_speed: Vector2f,
        is_tumble: bool,
    ) -> Self {
        let fly_top_angle_lw= WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("fly_top_angle_lw"));
        let fly_top_angle_hi= WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("fly_top_angle_hi"));
        let is_damage_fly_top = fly_top_angle_lw <= launch_radians && launch_radians <= fly_top_angle_hi;
        let ecb_bottom = *GroundModule::get_rhombus(defender_boma, true).add(1);
        let ecb_left =   *GroundModule::get_rhombus(defender_boma, true).add(2);
        let ecb_right =  *GroundModule::get_rhombus(defender_boma, true).add(3);
        let y_chara_speed = 0.0;
        let gravity = if is_damage_fly_top {
            WorkModule::get_param_float(defender_boma, hash40("air_accel_y"), hash40(""))
        } else {
            WorkModule::get_param_float(defender_boma, hash40("damage_fly_top_air_accel_y"), hash40(""))
        };
        let fall_speed = if is_damage_fly_top {
            WorkModule::get_param_float(defender_boma, hash40("air_speed_y_stable"), hash40(""))
        } else {
            WorkModule::get_param_float(defender_boma, hash40("damage_fly_top_speed_y_stable"), hash40(""))
        };
        let pos = Vector2f::new(ecb_bottom.x, ecb_bottom.y);
        let pos_prev = Vector2f::new(pos.x, pos.y);
        let damage_air_brake = WorkModule::get_param_float(defender_boma, hash40("common"), hash40("damage_air_brake"));
        let speed_up_mul = if WorkModule::is_flag(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP) {
            WorkModule::get_float(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG)
        } else {
            1.0
        };
        let is_tech_possible = false;
        let mut context = Self {
            defender_boma,
            fly_top_angle_lw,
            fly_top_angle_hi,
            ecb_bottom,
            ecb_left,
            ecb_right,
            knockback,
            hitstun,
            damage,
            sdi_mul,
            launch_radians,
            launch_speed,
            y_chara_speed,
            is_tumble,
            is_damage_fly_top,
            gravity,
            fall_speed,
            pos,
            pos_prev,
            damage_air_brake,
            speed_up_mul,
            is_tech_possible,
        };
        return context;
    }

    pub unsafe fn reset_angle(&mut self, launch_radians: f32) {
        // calculate values that depend on the new angle
        let mag = (self.launch_speed.y.powi(2) + self.launch_speed.x.powi(2)).sqrt();
        let launch_speed = Vector2f::new(
            launch_radians.cos() * mag,
            launch_radians.sin() * mag,
        );
        let is_damage_fly_top = self.fly_top_angle_lw <= launch_radians && launch_radians <= self.fly_top_angle_hi;
        let defender_boma = self.defender_boma;
        let gravity = if is_damage_fly_top {
            WorkModule::get_param_float(defender_boma, hash40("air_accel_y"), hash40(""))
        } else {
            WorkModule::get_param_float(defender_boma, hash40("damage_fly_top_air_accel_y"), hash40(""))
        };
        let fall_speed = if is_damage_fly_top {
            WorkModule::get_param_float(defender_boma, hash40("air_speed_y_stable"), hash40(""))
        } else {
            WorkModule::get_param_float(defender_boma, hash40("damage_fly_top_speed_y_stable"), hash40(""))
        };

        // update the context
        self.launch_radians = launch_radians;
        self.launch_speed = launch_speed;
        self.is_damage_fly_top = is_damage_fly_top;
        self.gravity = gravity;
        self.fall_speed = fall_speed;
    }

    pub unsafe fn initial_launch_collision_check(&mut self) {
        let defender_boma = self.defender_boma;
        let sdi_frame =     WorkModule::get_param_int(defender_boma, hash40("common"), hash40("hit_stop_delay_flick_frame"));
        let sdi_max_count = WorkModule::get_param_int(defender_boma, hash40("common"), hash40("hit_stop_delay_flick_max_count"));
        let base_sdi =      WorkModule::get_param_float(defender_boma, hash40("common"), hash40("hit_stop_delay_flick_mul"));
        let base_asdi =     WorkModule::get_param_float(defender_boma, hash40("common"), hash40("hit_stop_delay_auto_mul"));
        let hitlag_max =    WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("hitstop_frame_max"));
        let hitlag_add =    WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("hitstop_frame_add"));
        let hitlag_mul =    WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("hitstop_frame_mul"));
        let hitlag = (2.0 * (self.damage * hitlag_mul + hitlag_add)).clamp(0.0, hitlag_max).floor();
        let sdi_count = ((hitlag - 1.0) / (sdi_frame as f32)).clamp(0.0, sdi_max_count as f32).floor();
        let sdi_distance = (sdi_count * base_sdi + base_asdi) * self.sdi_mul;

        // check left wall tech
        let ecb_offset = self.ecb_left.x - self.ecb_bottom.x;
        if GroundModule::ray_check(
            defender_boma, 
            &self.pos, 
            &Vector2f{ x: -1.0 * sdi_distance + ecb_offset, y: 0.0},
            true
        ) == 1 {
            self.is_tech_possible = true;
            return;
        }

        // check right wall tech
        let ecb_offset = self.ecb_right.x - self.ecb_bottom.x;
        if GroundModule::ray_check(
            defender_boma, 
            &self.pos, 
            &Vector2f{ x: sdi_distance + ecb_offset, y: 0.0},
            true
        ) == 1 {
            self.is_tech_possible = true;
            return;
        }

        // check floor tech
        if self.pos.y - self.pos_prev.y < base_asdi * self.sdi_mul
        && GroundModule::ray_check(
            defender_boma, 
            &self.pos, 
            &Vector2f{ x: 0.0, y: sdi_distance},
            true
        ) == 1 {
            self.is_tech_possible = true;
            return;
        }
    }

    pub unsafe fn collision_check(&mut self) {
        let defender_boma = self.defender_boma;
        let diff = Vector2f::new(self.pos.x - self.pos_prev.x, self.pos.y - self.pos_prev.y);
        if GroundModule::ray_check(
            defender_boma, 
            &self.pos_prev, 
            &diff, 
            diff.y <= 0.0 // only check for platforms if going downwards
        ) == 1 {
            self.is_tech_possible = true;
            return;
        }
    }

    pub unsafe fn step(&mut self) {
        let kb_angle = self.launch_speed.y.atan2(self.launch_speed.x);
        let decay = Vector2f::new(
            self.damage_air_brake * kb_angle.cos().abs(),
            self.damage_air_brake * kb_angle.sin().abs()
        );

        self.pos_prev.x = self.pos.x;
        self.pos_prev.y = self.pos.y;
        self.pos.x += self.launch_speed.x;
        self.pos.y += self.launch_speed.y + self.y_chara_speed;
        if (self.launch_speed.x != 0.0) {
            let dir = f32::signum(self.launch_speed.x);
            self.launch_speed.x = f32::abs(self.launch_speed.x) - decay.x;
            if (self.launch_speed.x < 0.0) {
                self.launch_speed.x = 0.0;
            } else {
              self.launch_speed.x *= dir;
            }
        }

        if (self.launch_speed.y != 0.0) {
            let dir = f32::signum(self.launch_speed.y);
            self.launch_speed.y = f32::abs(self.launch_speed.y) - decay.y;
            if (self.launch_speed.y < 0.0) {
                self.launch_speed.y = 0.0;
            } else {
                self.launch_speed.y *= dir;
            }
        }
        self.y_chara_speed = f32::max(self.y_chara_speed - self.gravity, -self.fall_speed);
    }

    pub unsafe fn get_trajectory(&mut self) -> Vec<Vector2f> {
        let mut trajectory = Vec::new();
        for i in 0..self.hitstun.floor() as i32 {
            trajectory.push(Vector2f::new(self.pos.x, self.pos.y));
            self.step();
            if i == 0 {
                self.initial_launch_collision_check();
            }
            self.collision_check();
            if self.is_tech_possible {
                break;
            }
        }
        return trajectory;
    }

    pub unsafe fn is_finishing_hit(&mut self, false_angle_num_allowed: i32) -> bool {
        let defender_boma = self.defender_boma;
        let blastzones = get_dead_area();
        let kb_angle = self.launch_speed.y.atan2(self.launch_speed.x).to_degrees();
        let di_angle = WorkModule::get_param_float(defender_boma, hash40("common"), hash40("damage_fly_correction_max"));
        let min_di = kb_angle - di_angle;
        let max_di = kb_angle + di_angle;
        let step = (di_angle * 2.0) / (NUM_ANGLE_CHECK as f32);
        let mut false_angle_num = 0;
        let original_context = self.clone();
        for idx in 0..NUM_ANGLE_CHECK + 1 {
            // calc and update the DI angle
            let new_radians = (min_di + (idx as f32 * step)).to_radians();
            
            // reset everything to scratch
            *self = original_context.clone();
            self.reset_angle(new_radians);

            // check if it kills at this angle
            let trajectory = self.get_trajectory();
            let mut trajectory_kills = false;
            for (frame, pos) in trajectory.iter().enumerate() {
                if !blastzones.contains(pos.x, pos.y) {
                    // break early so we don't waste effort
                    trajectory_kills = true;
                    break;
                }
            }
            if !trajectory_kills {
                false_angle_num += 1;
            }
            if false_angle_num > false_angle_num_allowed {
                // return early so we don't waste effort
                return false;
            }
        }
        return true;
    }
}