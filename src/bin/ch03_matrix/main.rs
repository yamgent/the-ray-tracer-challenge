use the_ray_tracer_challenge::math::{assert_float_eq, Matrix4x4f};

fn main() {
    fn loose_compare_matrix4x4f(left: &Matrix4x4f, right: &Matrix4x4f) {
        const ACCEPTABLE_DELTA: f64 = 0.0001;

        (0..4).for_each(|r| {
            (0..4).for_each(|c| {
                let left = left.get(r, c);
                let right = right.get(r, c);
                assert!(
                    (left - right).abs() < ACCEPTABLE_DELTA,
                    "{} != {}, Cell: ({}, {})",
                    left,
                    right,
                    r,
                    c
                );
            })
        });
    }

    assert_float_eq(
        Matrix4x4f::identity().inverse().unwrap(),
        Matrix4x4f::identity(),
    );

    let m = Matrix4x4f::new([
        8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
    ]);
    loose_compare_matrix4x4f(&(m * m.inverse().unwrap()), &Matrix4x4f::identity());

    assert_float_eq(
        m.transpose().inverse().unwrap(),
        m.inverse().unwrap().transpose(),
    );

    println!("All passed!");
}
