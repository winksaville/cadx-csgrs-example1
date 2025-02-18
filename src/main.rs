use nalgebra::Vector3;

fn main() {
    // Alias the library’s generic CSG type with empty metadata:
    type CSG = csgrs::csg::CSG<()>;

    // Create two shapes:
    let cube = CSG::cube(2.0, 2.0, 2.0, None);  // 2×2×2 cube at origin, no metadata
    let sphere1_25 = CSG::sphere(1.25, 16, 8, None); // sphere of radius=1 at origin, no metadata
    let trans_sphere1_25 = sphere1_25.translate(Vector3::new(1.0, 1.0, 1.0));

    // Difference one from the other:
    let difference_result = cube.difference(&trans_sphere1_25);

    // Write the result as an ASCII STL:
    let stl = difference_result.to_stl_ascii("cube_minus_trans_sphere1_25");
    std::fs::write("cube_trans_sphere1_25_difference.stl", stl).unwrap();
}
