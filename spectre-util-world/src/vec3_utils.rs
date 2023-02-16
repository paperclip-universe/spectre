use glam::DVec3;

trait Moveable {
    /// Moves the
    fn up(&mut self);
    fn up(&mut self, force: i32);
}
