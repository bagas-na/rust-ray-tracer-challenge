use crate::{matrix::Matrix4, tuple::Tuple};

/// Creates a 3D translation matrix
pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    let mut matrix = Matrix4::identity();
    matrix.set(0, 3, x).ok();
    matrix.set(1, 3, y).ok();
    matrix.set(2, 3, z).ok();
    matrix
}

/// Creates a scaling matrix that works in 3D
pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
    let mut matrix = Matrix4::identity();
    matrix.set(0, 0, x).ok();
    matrix.set(1, 1, y).ok();
    matrix.set(2, 2, z).ok();
    matrix
}

/// Creates a rotation matrix around the x-axis clockwise
/// (left hand rule), Given an angle in radian
pub fn rotation_x(rad: f64) -> Matrix4 {
    Matrix4::from_tuples_by_row(
        Tuple::new(1., 0., 0., 0.),
        Tuple::new(0., rad.cos(), -rad.sin(), 0.),
        Tuple::new(0., rad.sin(), rad.cos(), 0.),
        Tuple::new(0., 0., 0., 1.),
    )
}

/// Creates a rotation matrix around the y-axis clockwise
/// (left hand rule), Given an angle in radian
pub fn rotation_y(rad: f64) -> Matrix4 {
    Matrix4::from_tuples_by_row(
        Tuple::new(rad.cos(), 0., rad.sin(), 0.),
        Tuple::new(0., 1.0, 0., 0.),
        Tuple::new(-rad.sin(), 0., rad.cos(), 0.),
        Tuple::new(0., 0., 0., 1.),
    )
}

/// Creates a rotation matrix around the z-axis clockwise
/// (left hand rule), Given an angle in radian
pub fn rotation_z(rad: f64) -> Matrix4 {
    Matrix4::from_tuples_by_row(
        Tuple::new(rad.cos(), -rad.sin(), 0., 0.),
        Tuple::new(rad.sin(), rad.cos(), 0., 0.),
        Tuple::new(0., 0., 0., 0.),
        Tuple::new(0., 0., 0., 1.),
    )
}

#[cfg(test)]
mod tests {
    use crate::{
        transform::{self},
        tuple::Tuple,
    };

    #[test]
    fn multiply_by_translation_matrix() {
        let transform = transform::translation(5.0, -3.0, 2.0);
        let point_p = Tuple::new_point(-3.0, 4.0, 5.0);
        assert_eq!(transform * point_p, Tuple::new_point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiply_by_inverse_translation_matrix() {
        let transform = transform::translation(5.0, -3.0, 2.0);
        let inverse = transform.inverse().unwrap();
        let point_p = Tuple::new_point(-3.0, 4.0, 5.0);
        assert_eq!(inverse * point_p, Tuple::new_point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = transform::translation(5.0, -3.0, 2.0);
        let vector_v = Tuple::new_vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * &vector_v, vector_v);
    }

    #[test]
    fn scaling_applied_to_a_point() {
        let transform = transform::scaling(2., 3., 4.);
        let point_p = Tuple::new_point(-4., 6., 8.);
        assert_eq!(transform * point_p, Tuple::new_point(-8., 18., 32.));
    }

    #[test]
    fn scaling_applied_to_a_vector() {
        let transform = transform::scaling(2., 3., 4.);
        let vector_v = Tuple::new_vector(-4., 6., 8.);
        assert_eq!(transform * vector_v, Tuple::new_vector(-8., 18., 32.));
    }

    #[test]
    fn apply_inverse_scaling_to_a_vector() {
        let transform = transform::scaling(2., 3., 4.);
        let inverse = transform.inverse().unwrap();
        let point_p = Tuple::new_point(-4., 6., 8.);
        assert_eq!(inverse * point_p, Tuple::new_point(-2., 2., 2.));
    }

    #[test]
    fn rotating_x_axis() {
        let point_p = Tuple::new_point(0., 1., 0.);
        let half_quarter = transform::rotation_x(std::f64::consts::PI / 4.0);
        let full_quarter = transform::rotation_x(std::f64::consts::PI / 2.0);

        assert_eq!(
            half_quarter * &point_p,
            Tuple::new_point(0.0, f64::sqrt(2.) / 2.0, f64::sqrt(2.) / 2.0)
        );
        assert_eq!(full_quarter * point_p, Tuple::new_point(0., 0., 1.));
    }

    #[test]
    fn rotating_x_axis_inverse() {
        let point_p = Tuple::new_point(0., 1., 0.);
        let half_quarter = transform::rotation_x(std::f64::consts::PI / 4.0);
        let inverse = half_quarter.inverse().unwrap();

        assert_eq!(
            inverse * point_p,
            Tuple::new_point(0., 1. / f64::sqrt(2.), -1. / f64::sqrt(2.))
        );
    }

        #[test]
    fn rotating_y_axis() {
        let point_p = Tuple::new_point(0., 0., 1.);
        let half_quarter = transform::rotation_y(std::f64::consts::PI / 4.0);
        let full_quarter = transform::rotation_y(std::f64::consts::PI / 2.0);

        assert_eq!(
            half_quarter * &point_p,
            Tuple::new_point(f64::sqrt(2.) / 2.0, 0.0, f64::sqrt(2.) / 2.0)
        );
        assert_eq!(full_quarter * point_p, Tuple::new_point(1., 0., 0.));
    }

            #[test]
    fn rotating_z_axis() {
        let point_p = Tuple::new_point(0., 1., 0.);
        let half_quarter = transform::rotation_z(std::f64::consts::PI / 4.0);
        let full_quarter = transform::rotation_z(std::f64::consts::PI / 2.0);

        assert_eq!(
            half_quarter * &point_p,
            Tuple::new_point(-f64::sqrt(2.) / 2.0, f64::sqrt(2.) / 2.0, 0.0)
        );
        assert_eq!(full_quarter * point_p, Tuple::new_point(-1., 0., 0.));
    }
}
