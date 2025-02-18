fn main() {
    // Alias the library’s generic CSG type with empty metadata:
    type CSG = csgrs::csg::CSG<()>;

    // Create two shapes:
    let cube = CSG::cube(2.0, 2.0, 2.0, None);  // 2×2×2 cube at origin, no metadata
    let sphere = CSG::sphere(1.0, 16, 8, None); // sphere of radius=1 at origin, no metadata

    // Difference one from the other:
    let difference_result = cube.difference(&sphere);

    // Write the result as an ASCII STL:
    let stl = difference_result.to_stl_ascii("cube_minus_sphere");
    std::fs::write("cube_sphere_difference.stl", stl).unwrap();
}
