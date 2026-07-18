use crate::body::Body;
use crate::vector3::Vector3;

pub fn compute_accelerations(bodies: &[Body], g: f64, softening: f64) -> Vec<Vector3> {
    let n = bodies.len();
    let mut accelerations = vec![Vector3::ZERO; n];
    let eps2 = softening * softening;

    for i in 0..n {
        let mut acc = Vector3::ZERO;
        for j in 0..n {
            if i == j {
                continue;
            }
            let delta = bodies[j].position - bodies[i].position;
            let dist_sq = delta.norm_squared() + eps2;
            let dist = dist_sq.sqrt();
            let factor = g * bodies[j].mass / (dist_sq * dist);
            acc += delta * factor;
        }
        accelerations[i] = acc;
    }
    accelerations
}