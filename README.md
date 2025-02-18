# Cad Experiment csgrs example 1

This is example 1 currently from the csgrs the code is:
```rust
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
```

The resulting file is `cube_sphere_difference.stl`. I then used used `f3d cube_sphere_difference.stl` to view the file and the result is shown below:

![cube_sphere_difference](./cube_sphere_difference.stl.png)

I then modifed the file so only the sphere is shown and the code is:
```rust
fn main() {
    // Alias the library’s generic CSG type with empty metadata:
    type CSG = csgrs::csg::CSG<()>;

    // Create two shapes:
    //let cube = CSG::cube(2.0, 2.0, 2.0, None);  // 2×2×2 cube at origin, no metadata
    let sphere = CSG::sphere(1.0, 16, 8, None); // sphere of radius=1 at origin, no metadata
    let stl = sphere.to_stl_ascii("sphere");
    std::fs::write("sphere.stl", stl).unwrap();

    // Difference one from the other:
    //let difference_result = cube.difference(&sphere);

    // Write the result as an ASCII STL:
    //let stl = difference_result.to_stl_ascii("cube_minus_sphere");
    //std::fs::write("cube_sphere_difference.stl", stl).unwrap();
}
```

![sphere](./sphere.stl.png)

The comment for the `let sphere = ..` says `// shpere of radius=1 at the origin`
that is not the case from my perspective based on the x, y, z axis I see in the
cube_sphere_difference.stl rendering.


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
