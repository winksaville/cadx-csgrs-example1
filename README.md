# Cad Experiment csgrs example 1

Change the code to take the sphere_trans and sphere_radius
from the command line so we can easily test various values.

```rust
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
```

## Usage

5 examples:


```
cargo run -- 0.00 1.00
```
![cargo run -- 0.00 1.00](./cube_trans0.00_sphere1.00_difference.stl.png)

```
cargo run -- 0.00 1.25
```
![cargo run -- 0.00 1.25](./cube_trans0.00_sphere1.25_difference.stl.png)

```
cargo run -- 0.10 1.25
```
![cargo run -- 0.10 1.25](./cube_trans0.10_sphere1.25_difference.stl.png)

```
cargo run -- 0.50 1.25
```
![cargo run -- 0.50 1.25](./cube_trans0.50_sphere1.25_difference.stl.png)

```
cargo run -- 1.00 1.25
```
![cargo run -- 1.00 1.25](./cube_trans1.00_sphere1.25_difference.stl.png)
 

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
