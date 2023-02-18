use glam::DVec3;

pub struct Facing {
    pub direction: DVec3,
    pub axis: Axis,
    pub axis_direction: AxisDirection,
}

pub enum FacingDirection {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl FacingDirection {
    pub fn get_facing(&self) -> Facing {
        match &self {
            FacingDirection::Down => Facing {
                direction: DVec3::new(0.0, -1.0, 0.0),
                axis: Axis::Y,
                axis_direction: AxisDirection::Negative,
            },
            FacingDirection::Up => Facing {
                direction: DVec3::new(0.0, 1.0, 0.0),
                axis: Axis::Y,
                axis_direction: AxisDirection::Positive,
            },
            FacingDirection::North => Facing {
                direction: DVec3::new(0.0, 0.0, -1.0),
                axis: Axis::Z,
                axis_direction: AxisDirection::Negative,
            },
            FacingDirection::South => Facing {
                direction: DVec3::new(0.0, 0.0, 1.0),
                axis: Axis::Z,
                axis_direction: AxisDirection::Positive,
            },
            FacingDirection::West => Facing {
                direction: DVec3::new(-1.0, 0.0, 0.0),
                axis: Axis::X,
                axis_direction: AxisDirection::Negative,
            },
            FacingDirection::East => Facing {
                direction: DVec3::new(1.0, 0.0, 0.0),
                axis: Axis::X,
                axis_direction: AxisDirection::Positive,
            },
        }
    }
}

pub enum Axis {
    X,
    Y,
    Z,
}
pub enum AxisDirection {
    Negative,
    Positive,
}
