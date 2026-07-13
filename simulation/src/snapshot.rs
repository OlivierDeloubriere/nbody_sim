use crate::vector3::Vector3;

pub struct Snapshot {
    pub time: f64,
    pub positions: Vec<Vector3>,
    pub velocities: Vec<Vector3>,
}