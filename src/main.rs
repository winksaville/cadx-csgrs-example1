use std::env;

use nalgebra::Vector3;

fn main() {
    // Alias the library’s generic CSG type with empty metadata:
    type CSG = csgrs::csg::CSG<()>;

    let args: Vec<String> = env::args().collect();
    let sphere_trans = args[1].parse::<f64>().unwrap();
    let sphere_radius = args[2].parse::<f64>().unwrap();

    // Create two shapes:
    let cube = CSG::cube(2.0, 2.0, 2.0, None);  // 2×2×2 cube at origin, no metadata
    let sphere = CSG::sphere(sphere_radius, 16, 8, None); // sphere of radius=1 at origin, no metadata
    let trans_sphere = sphere.translate(Vector3::new(sphere_trans, sphere_trans, sphere_trans));

    // Difference one from the other:
    let difference_result = cube.difference(&trans_sphere);

    // Write the result as an ASCII STL:
    let name = &format!("cube_trans{:0.2}_sphere{:0.2}", sphere_trans, sphere_radius);
    let stl = difference_result.to_stl_ascii(name);
    std::fs::write(name.to_owned() + "_difference.stl", stl).unwrap();
}
