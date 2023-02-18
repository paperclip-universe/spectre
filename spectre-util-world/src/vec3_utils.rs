use crate::facing::FacingDirection;
use glam::DVec3;

/// A trait for bumping a Vec3 in a direction.
trait Bumpable {
    /// Moves the Vec3 up by one.
    fn up(&mut self);
    /// Moves the Vec3 up by the specified force.
    fn up_with_force(&mut self, force: i32);
    /// Moves the Vec3 down by one.
    fn down(&mut self);
    /// Moves the Vec3 down by the specified force.
    fn down_with_force(&mut self, force: i32);
    /// Moves the Vec3 north by one.
    fn north(&mut self);
    /// Moves the Vec3 north by the specified force.
    fn north_with_force(&mut self, force: i32);
    /// Moves the Vec3 south by one.
    fn south(&mut self);
    /// Moves the Vec3 south by the specified force.
    fn south_with_force(&mut self, force: i32);
    /// Moves the Vec3 west by one.
    fn west(&mut self);
    /// Moves the Vec3 west by the specified force.
    fn west_with_force(&mut self, force: i32);
    /// Moves the Vec3 east by one.
    fn east(&mut self);
    /// Moves the Vec3 east by the specified force.
    fn east_with_force(&mut self, force: i32);
    /// Moves the Vec3 in the specified direction by one.
    fn bump(&mut self, direction: FacingDirection);
    /// Moves the Vec3 in the specified direction by the specified force.
    fn bump_with_force(&mut self, direction: FacingDirection, force: i32);
}

impl Bumpable for DVec3 {
    fn up(&mut self) {
        self.bump(FacingDirection::Up)
    }

    fn up_with_force(&mut self, force: i32) {
        self.bump_with_force(FacingDirection::Up, force)
    }

    fn down(&mut self) {
        self.bump(FacingDirection::Down)
    }

    fn down_with_force(&mut self, force: i32) {
        self.bump_with_force(FacingDirection::Down, force)
    }

    fn north(&mut self) {
        self.bump(FacingDirection::North)
    }

    fn north_with_force(&mut self, force: i32) {
        self.bump_with_force(FacingDirection::North, force)
    }

    fn south(&mut self) {
        self.bump(FacingDirection::South)
    }

    fn south_with_force(&mut self, force: i32) {
        self.bump_with_force(FacingDirection::South, force)
    }

    fn west(&mut self) {
        self.bump(FacingDirection::West)
    }

    fn west_with_force(&mut self, force: i32) {
        self.bump_with_force(FacingDirection::West, force)
    }

    fn east(&mut self) {
        self.bump(FacingDirection::East)
    }

    fn east_with_force(&mut self, force: i32) {
        self.bump_with_force(FacingDirection::East, force)
    }

    #[inline(always)]
    fn bump(&mut self, direction: FacingDirection) {
        self.bump_with_force(direction, 1)
    }

    #[inline(always)]
    fn bump_with_force(&mut self, direction: FacingDirection, force: i32) {
        *self += direction.get_facing().direction * force as f64;
    }
}
