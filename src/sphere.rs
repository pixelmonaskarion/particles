use std::f32::consts::PI;

use crate::height_map::Vertex;

//https://gist.github.com/Pikachuxxxx/5c4c490a7d7679824e0e18af42918efc



pub fn generate_sphere_smooth_continued(radius: f32, mut latitudes: u32, mut longitudes: u32, color: [f32; 3], position: [f32; 3], vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    let index_offset = vertices.len() as u32;

    if longitudes < 3 {
        longitudes = 3;
    }
    if latitudes < 2 {
         latitudes = 2;
    }

    let length_inv = 1.0 / radius;

    let delta_latitude = PI / latitudes as f32;
    let delta_longitude = PI / longitudes as f32;
    let mut latitude_angle;
    let mut longitude_angle;

    // Compute all vertices first except normals
    for i in 0..latitudes {
        latitude_angle = PI / 2. - i as f32 * delta_latitude; /* Starting -pi/2 to pi/2 */
        let xy = radius * latitude_angle.cos();    /* r * cos(phi) */
        let z = radius * latitude_angle.sin();     /* r * sin(phi )*/

        /*
         * We add (latitudes + 1) vertices per longitude because of equator,
         * the North pole and South pole are not counted here, as they overlap.
         * The first and last vertices have same position and normal, but
         * different tex coords.
         */
        for j in 0..longitudes {
            longitude_angle = (j as f32 * delta_longitude) * 2.;

            let mut vertex = Vertex {
                position: [
                    xy * longitude_angle.cos() + position[0],
                    xy * longitude_angle.sin() + position[1],
                    z + position[2],
                ],
                color: color.clone(),
                normal: [0., 1., 0.],
            };
            // vertex.s = (float)j/longitudes;             /* s */
            // vertex.t = (float)i/latitudes;              /* t */

            // normalized vertex normal
            vertex.normal[0] = vertex.position[0] * length_inv;
            vertex.normal[1] = vertex.position[1] * length_inv;
            vertex.normal[2] = vertex.position[2] * length_inv;
            vertices.push(vertex);
        }
    }

    /*
     *  Indices
     *  k1--k1+1
     *  |  / |
     *  | /  |
     *  k2--k2+1
     */
    let mut k1: u32;
    let mut k2: u32;
    for i in 0..latitudes {
        k1 = i * (longitudes);
        k2 = k1 + longitudes ;
        // 2 Triangles per latitude block excluding the first and last longitudes blocks
        for _ in 0..longitudes {
            k1 += 1;
            k2 += 1;
            if i != 0 {
                indices.push(k1 + index_offset);
                indices.push(k2 + index_offset);
                indices.push(k1 + 1 + index_offset);
            }

            if i != (latitudes - 1) {
                indices.push(k1 + 1 + index_offset);
                indices.push(k2 + index_offset);
                indices.push(k2 + 1 + index_offset);
            }
        }
    }
}

pub fn generate_sphere_smooth(radius: f32, latitudes: u32, longitudes: u32, color: [f32; 3]) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = vec![];
    let mut indices: Vec<u32> = vec![];

    generate_sphere_smooth_continued(radius, latitudes, longitudes, color, [0., 0., 0.], &mut vertices, &mut indices);

    (vertices, indices)
}