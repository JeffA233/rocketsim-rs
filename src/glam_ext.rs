pub use glam;

#[cfg(target_arch = "x86")]
use core::arch::x86::__m128;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__m128;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
use core::simd::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type F32x4 = __m128;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
type F32x4 = f32x4;

use core::pin::Pin;
use glam::{EulerRot, Mat3, Mat3A, Quat, Vec3, Vec3A, Vec4};

use crate::{
    math::{Angle, RotMat, Vec3 as Vec3R},
    sim::{
        Arena, BallHitInfo, BallState, BoostPadState, CarConfig, CarControls, CarState, GameMode, HeatseekerInfo, Team,
        WheelPairConfig,
    },
    BoostPad, CarInfo, GameState,
};

impl From<RotMat> for Mat3A {
    #[inline]
    fn from(value: RotMat) -> Self {
        Self::from_cols(value.forward.into(), value.right.into(), value.up.into())
    }
}

impl From<RotMat> for Mat3 {
    #[inline]
    fn from(value: RotMat) -> Self {
        Self::from_cols(value.forward.into(), value.right.into(), value.up.into())
    }
}

impl From<Mat3A> for RotMat {
    #[inline]
    fn from(value: Mat3A) -> Self {
        Self {
            forward: value.x_axis.into(),
            right: value.y_axis.into(),
            up: value.z_axis.into(),
        }
    }
}

impl From<Mat3> for RotMat {
    #[inline]
    fn from(value: Mat3) -> Self {
        Self {
            forward: value.x_axis.into(),
            right: value.y_axis.into(),
            up: value.z_axis.into(),
        }
    }
}

impl From<Angle> for Mat3A {
    #[inline]
    fn from(value: Angle) -> Self {
        Self::from_quat(Quat::from(value))
    }
}

impl From<Angle> for Mat3 {
    #[inline]
    fn from(value: Angle) -> Self {
        Self::from_quat(Quat::from(value))
    }
}

impl From<Angle> for RotMat {
    #[inline]
    fn from(value: Angle) -> Self {
        value.to_rotmat()
    }
}

impl From<Angle> for Quat {
    #[inline]
    fn from(value: Angle) -> Self {
        Self::from_euler(EulerRot::XYZ, value.roll, value.pitch, value.yaw)
    }
}

impl From<&Mat3A> for Angle {
    #[inline]
    fn from(value: &Mat3A) -> Self {
        Angle::from(Quat::from_mat3a(value))
    }
}

impl From<&Mat3> for Angle {
    #[inline]
    fn from(value: &Mat3) -> Self {
        Angle::from(Quat::from_mat3(value))
    }
}

impl From<Quat> for Angle {
    #[inline]
    fn from(value: Quat) -> Self {
        let (roll, pitch, yaw) = value.to_euler(EulerRot::XYZ);
        Self { yaw, pitch, roll }
    }
}

impl From<Vec3R> for Vec3A {
    #[inline]
    fn from(value: Vec3R) -> Self {
        Vec3A::from(F32x4::from(value.to_glam()))
    }
}

impl From<Vec3A> for Vec3R {
    #[inline]
    fn from(value: Vec3A) -> Self {
        Self::from_glam(Vec4::from(F32x4::from(value)))
    }
}

impl From<Vec3R> for Vec3 {
    #[inline]
    fn from(value: Vec3R) -> Self {
        Vec3::new(value.x, value.y, value.z)
    }
}

impl From<Vec3> for Vec3R {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

pub trait FromRotMat {
    fn from_rotmat(rot_mat: RotMat) -> Self;
}

impl FromRotMat for Quat {
    #[inline]
    fn from_rotmat(rot_mat: RotMat) -> Self {
        Quat::from_mat3(&Mat3::from(rot_mat))
    }
}

impl Vec3R {
    #[inline]
    #[must_use]
    pub const fn to_glam(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self._w)
    }

    #[inline]
    #[must_use]
    pub const fn from_glam(vec: Vec4) -> Self {
        let [x, y, z, w] = vec.to_array();
        Self { x, y, z, _w: w }
    }
}

impl RotMat {
    #[inline]
    #[must_use]
    pub fn from_quat(quat: Quat) -> Self {
        Self::from(Mat3::from_quat(quat))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoostPadA {
    pub is_big: bool,
    pub position: Vec3A,
    pub state: BoostPadState,
}

impl From<BoostPad> for BoostPadA {
    #[inline]
    fn from(value: BoostPad) -> Self {
        Self {
            is_big: value.is_big,
            position: value.position.into(),
            state: value.state,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BallHitInfoA {
    pub is_valid: bool,
    pub relative_pos_on_ball: Vec3A,
    pub ball_pos: Vec3A,
    pub extra_hit_vel: Vec3A,
    pub tick_count_when_hit: u64,
    pub tick_count_when_extra_impulse_applied: u64,
}

impl From<BallHitInfo> for BallHitInfoA {
    #[inline]
    fn from(value: BallHitInfo) -> Self {
        Self {
            is_valid: value.is_valid,
            relative_pos_on_ball: value.relative_pos_on_ball.into(),
            ball_pos: value.ball_pos.into(),
            extra_hit_vel: value.extra_hit_vel.into(),
            tick_count_when_hit: value.tick_count_when_hit,
            tick_count_when_extra_impulse_applied: value.tick_count_when_extra_impulse_applied,
        }
    }
}

impl From<BallHitInfoA> for BallHitInfo {
    #[inline]
    fn from(value: BallHitInfoA) -> Self {
        Self {
            is_valid: value.is_valid,
            relative_pos_on_ball: value.relative_pos_on_ball.into(),
            ball_pos: value.ball_pos.into(),
            extra_hit_vel: value.extra_hit_vel.into(),
            tick_count_when_hit: value.tick_count_when_hit,
            tick_count_when_extra_impulse_applied: value.tick_count_when_extra_impulse_applied,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BallA {
    pub update_counter: u64,
    pub pos: Vec3A,
    pub rot_mat: Mat3A,
    pub vel: Vec3A,
    pub ang_vel: Vec3A,
    pub hs_info: HeatseekerInfo,
}

impl Default for BallA {
    #[inline]
    fn default() -> Self {
        Self {
            update_counter: 0,
            pos: Vec3A::new(0., 0., 93.15),
            rot_mat: Mat3A::IDENTITY,
            vel: Vec3A::default(),
            ang_vel: Vec3A::default(),
            hs_info: HeatseekerInfo::default(),
        }
    }
}

impl From<BallState> for BallA {
    #[inline]
    fn from(value: BallState) -> Self {
        Self {
            update_counter: value.update_counter,
            pos: value.pos.into(),
            rot_mat: value.rot_mat.into(),
            vel: value.vel.into(),
            ang_vel: value.ang_vel.into(),
            hs_info: value.hs_info,
        }
    }
}

impl From<BallA> for BallState {
    #[inline]
    fn from(value: BallA) -> Self {
        Self {
            update_counter: value.update_counter,
            pos: value.pos.into(),
            rot_mat: value.rot_mat.into(),
            vel: value.vel.into(),
            ang_vel: value.ang_vel.into(),
            hs_info: value.hs_info,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WheelPairConfigA {
    pub wheel_radius: f32,
    pub suspension_rest_length: f32,
    pub connection_point_offset: Vec3A,
}

impl From<WheelPairConfig> for WheelPairConfigA {
    #[inline]
    fn from(value: WheelPairConfig) -> Self {
        Self {
            wheel_radius: value.wheel_radius,
            suspension_rest_length: value.suspension_rest_length,
            connection_point_offset: value.connection_point_offset.into(),
        }
    }
}

impl From<WheelPairConfigA> for WheelPairConfig {
    #[inline]
    fn from(value: WheelPairConfigA) -> Self {
        Self {
            wheel_radius: value.wheel_radius,
            suspension_rest_length: value.suspension_rest_length,
            connection_point_offset: value.connection_point_offset.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CarConfigA {
    pub hitbox_size: Vec3A,
    pub hitbox_pos_offset: Vec3A,
    pub front_wheels: WheelPairConfigA,
    pub back_wheels: WheelPairConfigA,
    pub dodge_deadzone: f32,
}

impl From<CarConfig> for CarConfigA {
    #[inline]
    fn from(value: CarConfig) -> Self {
        Self {
            hitbox_size: value.hitbox_size.into(),
            hitbox_pos_offset: value.hitbox_pos_offset.into(),
            front_wheels: value.front_wheels.into(),
            back_wheels: value.back_wheels.into(),
            dodge_deadzone: value.dodge_deadzone,
        }
    }
}

impl From<CarConfigA> for CarConfig {
    #[inline]
    fn from(value: CarConfigA) -> Self {
        Self {
            hitbox_size: value.hitbox_size.into(),
            hitbox_pos_offset: value.hitbox_pos_offset.into(),
            front_wheels: value.front_wheels.into(),
            back_wheels: value.back_wheels.into(),
            dodge_deadzone: value.dodge_deadzone,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CarStateA {
    pub update_counter: u64,
    pub pos: Vec3A,
    pub rot_mat: Mat3A,
    pub vel: Vec3A,
    pub ang_vel: Vec3A,
    pub is_on_ground: bool,
    pub has_jumped: bool,
    pub has_double_jumped: bool,
    pub has_flipped: bool,
    pub last_rel_dodge_torque: Vec3A,
    pub jump_time: f32,
    pub flip_time: f32,
    pub is_flipping: bool,
    pub is_jumping: bool,
    pub air_time_since_jump: f32,
    pub boost: f32,
    pub time_spent_boosting: f32,
    pub is_supersonic: bool,
    pub supersonic_time: f32,
    pub handbrake_val: f32,
    pub is_auto_flipping: bool,
    pub auto_flip_timer: f32,
    pub auto_flip_torque_scale: f32,
    pub has_contact: bool,
    pub contact_normal: Vec3A,
    pub other_car_id: u32,
    pub cooldown_timer: f32,
    pub is_demoed: bool,
    pub demo_respawn_timer: f32,
    pub ball_hit_info: BallHitInfoA,
    pub last_controls: CarControls,
}

impl Default for CarStateA {
    #[inline]
    fn default() -> Self {
        Self {
            update_counter: 0,
            pos: Vec3A::new(0., 0., 17.),
            rot_mat: Mat3A::IDENTITY,
            vel: Vec3A::default(),
            ang_vel: Vec3A::default(),
            is_on_ground: true,
            has_jumped: false,
            has_double_jumped: false,
            has_flipped: false,
            last_rel_dodge_torque: Vec3A::default(),
            jump_time: 0.,
            flip_time: 0.,
            is_flipping: false,
            is_jumping: false,
            air_time_since_jump: 0.,
            boost: 100. / 3.,
            time_spent_boosting: 0.,
            is_supersonic: false,
            supersonic_time: 0.,
            handbrake_val: 0.,
            is_auto_flipping: false,
            auto_flip_timer: 0.,
            auto_flip_torque_scale: 0.,
            has_contact: false,
            contact_normal: Vec3A::default(),
            other_car_id: 0,
            cooldown_timer: 0.,
            is_demoed: false,
            demo_respawn_timer: 0.,
            ball_hit_info: BallHitInfoA::default(),
            last_controls: CarControls::default(),
        }
    }
}

impl From<CarState> for CarStateA {
    #[inline]
    fn from(value: CarState) -> Self {
        Self {
            update_counter: value.update_counter,
            pos: value.pos.into(),
            rot_mat: value.rot_mat.into(),
            vel: value.vel.into(),
            ang_vel: value.ang_vel.into(),
            is_on_ground: value.is_on_ground,
            has_jumped: value.has_jumped,
            has_double_jumped: value.has_double_jumped,
            has_flipped: value.has_flipped,
            last_rel_dodge_torque: value.last_rel_dodge_torque.into(),
            jump_time: value.jump_time,
            flip_time: value.flip_time,
            is_flipping: value.is_flipping,
            is_jumping: value.is_jumping,
            air_time_since_jump: value.air_time_since_jump,
            boost: value.boost,
            time_spent_boosting: value.time_spent_boosting,
            is_supersonic: value.is_supersonic,
            supersonic_time: value.supersonic_time,
            handbrake_val: value.handbrake_val,
            is_auto_flipping: value.is_auto_flipping,
            auto_flip_timer: value.auto_flip_timer,
            auto_flip_torque_scale: value.auto_flip_torque_scale,
            has_contact: value.has_contact,
            contact_normal: value.contact_normal.into(),
            other_car_id: value.other_car_id,
            cooldown_timer: value.cooldown_timer,
            is_demoed: value.is_demoed,
            demo_respawn_timer: value.demo_respawn_timer,
            ball_hit_info: value.ball_hit_info.into(),
            last_controls: value.last_controls,
        }
    }
}

impl From<CarStateA> for CarState {
    #[inline]
    fn from(value: CarStateA) -> Self {
        Self {
            update_counter: value.update_counter,
            pos: value.pos.into(),
            rot_mat: value.rot_mat.into(),
            vel: value.vel.into(),
            ang_vel: value.ang_vel.into(),
            is_on_ground: value.is_on_ground,
            has_jumped: value.has_jumped,
            has_double_jumped: value.has_double_jumped,
            has_flipped: value.has_flipped,
            last_rel_dodge_torque: value.last_rel_dodge_torque.into(),
            jump_time: value.jump_time,
            flip_time: value.flip_time,
            is_flipping: value.is_flipping,
            is_jumping: value.is_jumping,
            air_time_since_jump: value.air_time_since_jump,
            boost: value.boost,
            time_spent_boosting: value.time_spent_boosting,
            is_supersonic: value.is_supersonic,
            supersonic_time: value.supersonic_time,
            handbrake_val: value.handbrake_val,
            is_auto_flipping: value.is_auto_flipping,
            auto_flip_timer: value.auto_flip_timer,
            auto_flip_torque_scale: value.auto_flip_torque_scale,
            has_contact: value.has_contact,
            contact_normal: value.contact_normal.into(),
            other_car_id: value.other_car_id,
            cooldown_timer: value.cooldown_timer,
            is_demoed: value.is_demoed,
            demo_respawn_timer: value.demo_respawn_timer,
            ball_hit_info: value.ball_hit_info.into(),
            last_controls: value.last_controls,
        }
    }
}

impl CarStateA {
    #[inline]
    #[must_use]
    /// Returns the other Car that this Car is currently contacting, if any
    pub fn get_contacting_car(&self, arena: Pin<&mut Arena>) -> Option<Self> {
        if self.other_car_id == 0 {
            None
        } else {
            Some(arena.get_car(self.other_car_id).into())
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CarInfoA {
    pub id: u32,
    pub team: Team,
    pub state: CarStateA,
    pub config: CarConfigA,
}

impl From<CarInfo> for CarInfoA {
    #[inline]
    fn from(value: CarInfo) -> Self {
        Self {
            id: value.id,
            team: value.team,
            state: value.state.into(),
            config: value.config.into(),
        }
    }
}

impl From<CarInfoA> for CarInfo {
    #[inline]
    fn from(value: CarInfoA) -> Self {
        Self {
            id: value.id,
            team: value.team,
            state: value.state.into(),
            config: value.config.into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct GameStateA {
    pub tick_rate: f32,
    pub tick_count: u64,
    pub game_mode: GameMode,
    pub cars: Vec<CarInfoA>,
    pub ball: BallA,
    pub pads: Vec<BoostPadA>,
}

impl From<GameState> for GameStateA {
    #[inline]
    fn from(value: GameState) -> Self {
        Self {
            tick_rate: value.tick_rate,
            tick_count: value.tick_count,
            game_mode: value.game_mode,
            cars: value.cars.into_iter().map(CarInfoA::from).collect(),
            ball: value.ball.into(),
            pads: value.pads.into_iter().map(BoostPadA::from).collect(),
        }
    }
}

impl GameState {
    #[must_use]
    pub fn to_glam(self) -> GameStateA {
        self.into()
    }
}
