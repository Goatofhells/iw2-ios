#![allow(non_camel_case_types)]

use macroquad::prelude::*;

pub const walls: [(Vec3, Vec3); 7] = [
    (vec3(-10., 2., 0.), vec3(0.5, 4., 24.)),
    (vec3(10., 2., 0.), vec3(0.5, 4., 24.)),
    (vec3(0., 2., -12.), vec3(20., 4., 0.5)),
    (vec3(0., 2., 12.), vec3(20., 4., 0.5)),
    (vec3(-4., 0.75, -1.), vec3(3., 1.5, 2.)),
    (vec3(4., 0.75, -4.), vec3(3., 1.5, 2.)),
    (vec3(0., 0.5, -7.), vec3(2., 1., 1.)),
];
pub const marks: [Vec3; 5] = [
    vec3(-7., 1.6, -8.), vec3(-3., 1.6, -10.), vec3(0., 2., -10.),
    vec3(3., 1.6, -10.), vec3(7., 1.6, -8.),
];

#[derive(Default)]
pub struct input {
    pub walk: Vec2,
    pub look: Vec2,
    pub fire: bool,
    pub reload: bool,
    pub jump: bool,
    pub crouch: bool,
    pub sprint: bool,
    pub aim: bool,
}

pub struct state {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub vel: f32,
    pub low: bool,
    pub ammo: u32,
    pub reload: f32,
    pub cool: f32,
    pub flash: f32,
    pub hit: f32,
    pub score: u32,
    pub shots: u32,
    pub time: f32,
    pub dead: [f32; 5],
    pub beam: Vec3,
}

impl Default for state {
    fn default() -> Self {
        Self { pos: vec3(0., 0., 8.), yaw: 0., pitch: 0., vel: 0., low: false,
            ammo: 12, reload: 0., cool: 0., flash: 0., hit: 0., score: 0,
            shots: 0, time: 60., dead: [0.; 5], beam: Vec3::ZERO }
    }
}

pub fn hit(pos: Vec3, dir: Vec3, mid: Vec3, size: Vec3) -> Option<f32> {
    let mut near: f32 = 0.;
    let mut far: f32 = 100.;
    for idx in 0..3 {
        let lo = mid[idx] - size[idx] * 0.5;
        let hi = mid[idx] + size[idx] * 0.5;
        if dir[idx].abs() < 0.00001 {
            if pos[idx] < lo || pos[idx] > hi { return None; }
        } else {
            let a = (lo - pos[idx]) / dir[idx];
            let b = (hi - pos[idx]) / dir[idx];
            near = near.max(a.min(b));
            far = far.min(a.max(b));
            if near > far { return None; }
        }
    }
    Some(near)
}

impl state {
    pub fn step(&mut self, dt: f32, cmd: input) {
        if !dt.is_finite() || dt <= 0. || self.time <= 0. { return; }
        let dt = dt.min(0.05).min(self.time);
        self.time = (self.time - dt).max(0.);
        self.yaw += cmd.look.x;
        self.pitch = (self.pitch + cmd.look.y).clamp(-1.3, 1.3);
        if cmd.crouch { self.low = !self.low; }
        if cmd.jump && self.pos.y <= 0.001 { self.vel = 5.; }
        self.vel -= 14. * dt;
        self.pos.y = (self.pos.y + self.vel * dt).max(0.);
        if self.pos.y == 0. { self.vel = 0.; }
        let fwd = vec3(self.yaw.sin(), 0., -self.yaw.cos());
        let side = vec3(self.yaw.cos(), 0., self.yaw.sin());
        let walk = cmd.walk.clamp_length_max(1.);
        let speed = if self.low { 1.8 } else if cmd.aim { 2.2 } else if cmd.sprint { 6. } else { 3.5 };
        let delta = (side * walk.x + fwd * walk.y) * speed * dt;
        for idx in [0, 2] {
            let old = self.pos[idx];
            self.pos[idx] += delta[idx];
            for (mid, size) in walls {
                if self.pos.x > mid.x - size.x * 0.5 - 0.3
                    && self.pos.x < mid.x + size.x * 0.5 + 0.3
                    && self.pos.z > mid.z - size.z * 0.5 - 0.3
                    && self.pos.z < mid.z + size.z * 0.5 + 0.3
                    && self.pos.y < mid.y + size.y * 0.5 {
                    self.pos[idx] = old;
                }
            }
        }
        self.cool = (self.cool - dt).max(0.);
        self.flash = (self.flash - dt).max(0.);
        self.hit = (self.hit - dt).max(0.);
        for val in &mut self.dead { *val = (*val - dt).max(0.); }
        if self.reload > 0. {
            self.reload = (self.reload - dt).max(0.);
            if self.reload == 0. { self.ammo = 12; }
        }
        if cmd.reload && self.ammo < 12 && self.reload == 0. { self.reload = 1.2; }
        if cmd.fire && self.cool == 0. && self.reload == 0. && self.ammo > 0. {
            self.ammo -= 1;
            self.shots += 1;
            self.cool = 0.18;
            self.flash = 0.07;
            let eye = self.pos + vec3(0., if self.low { 0.9 } else { 1.65 }, 0.);
            let dir = vec3(self.yaw.sin() * self.pitch.cos(), self.pitch.sin(), -self.yaw.cos() * self.pitch.cos());
            let mut dist: f32 = 100.;
            let mut pick = None;
            for (mid, size) in walls {
                if let Some(len) = hit(eye, dir, mid, size) { dist = dist.min(len); }
            }
            if dir.y < -0.0001 { dist = dist.min(-eye.y / dir.y); }
            for (idx, mid) in marks.iter().enumerate() {
                if self.dead[idx] > 0. { continue; }
                if let Some(len) = hit(eye, dir, *mid, vec3(0.8, 1.2, 0.2)) {
                    if len < dist { dist = len; pick = Some(idx); }
                }
            }
            self.beam = eye + dir * dist;
            if let Some(idx) = pick {
                self.dead[idx] = 1.8;
                self.score += 1;
                self.hit = 0.18;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rays() {
        assert_eq!(hit(vec3(0., 0., 5.), vec3(0., 0., -1.), Vec3::ZERO, vec3(2., 2., 2.)), Some(4.));
        assert_eq!(hit(vec3(3., 0., 5.), vec3(0., 0., -1.), Vec3::ZERO, vec3(2., 2., 2.)), None);
        assert_eq!(hit(vec3(0., 0., 5.), vec3(0., 0., 1.), Vec3::ZERO, vec3(2., 2., 2.)), None);
    }

    #[test]
    fn rules() {
        let mut game = state::default();
        game.pos = vec3(0., 0., -4.);
        game.pitch = (0.35_f32 / 6.).atan();
        game.step(0.01, input { fire: true, ..Default::default() });
        assert_eq!((game.score, game.ammo, game.shots), (1, 11, 1));
        game.step(0.01, input { fire: true, ..Default::default() });
        assert_eq!(game.shots, 1);
        game.step(0.01, input { reload: true, ..Default::default() });
        for _ in 0..25 { game.step(0.05, input::default()); }
        assert_eq!(game.ammo, 12);
        game.pos = vec3(0., 0., 8.);
        for _ in 0..200 { game.step(0.05, input { walk: vec2(1., 0.), sprint: true, ..Default::default() }); }
        assert!(game.pos.x <= 9.45);
        game.step(0.05, input { jump: true, ..Default::default() });
        assert!(game.pos.y > 0.);
        for _ in 0..30 { game.step(0.05, input::default()); }
        assert_eq!(game.pos.y, 0.);
        game.time = 0.;
        let pos = game.pos;
        game.step(0.05, input { walk: Vec2::ONE, fire: true, ..Default::default() });
        assert_eq!(game.pos, pos);
    }

    #[test]
    fn cover() {
        let mut game = state::default();
        game.pos = vec3(0., 0., -4.);
        game.low = true;
        game.step(0.01, input { fire: true, ..Default::default() });
        assert_eq!(game.score, 0);
        assert!(game.beam.z > -7.);
        game.step(f32::NAN, input { walk: Vec2::ONE, ..Default::default() });
        assert!(game.pos.is_finite());
    }
}
