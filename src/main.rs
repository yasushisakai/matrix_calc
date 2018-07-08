extern crate csv;
extern crate geometry;

use std::error::Error;
use std::io;
use std::process;

use csv::StringRecord;

use geometry::matrix3::Mat3;
use geometry::quaternion::Quat;
use geometry::vector::Vec3;

const DATA_START: usize = 2;
const DATA_CHUNK: usize = 6;

fn read_csv() -> Result<(), Box<Error>> {
    let mut next_limit = 0;
    let mut time_step = 0;
    let mut is_before = true;
    let mut is_first_plane = true;

    let mut six_pts: [Vec3; 6] = [
        Vec3::zero(),
        Vec3::zero(),
        Vec3::zero(),
        Vec3::zero(),
        Vec3::zero(),
        Vec3::zero(),
    ];
    let mut means: [Vec3; 2] = [Vec3::zero(), Vec3::zero()];
    let mut normals: [Vec3; 2] = [Vec3::zero(), Vec3::zero()];

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

    for (i, result) in reader.records().enumerate() {
        if i < DATA_START {
            continue;
        }; // headers

        let _record = result?;
        if i >= next_limit {
            next_limit = i + DATA_CHUNK;
            if is_before {
                println!("{}min,", time_step * 5 + 5);
            } else {
            }

            is_before = !is_before;
            if is_before {
                time_step += 1
            };
        }

        let mod6 = (i - DATA_START) % DATA_CHUNK;

        if mod6 % 3 == 0 {
            if is_first_plane {
                // println!("      abc");
            } else {
                // println!("      def");
            }
            is_first_plane = !is_first_plane;
        }

        let o = get_mean_coordinate(_record);

        match o {
            Some(v) => six_pts[mod6] = v,
            None => println!("wrong happend to get mean coordinate"),
        }

        if mod6 == 5 {
            // println!("{}, {}, {}", three_pts[0].x(),three_pts[1].x(),three_pts[2].x())
            // FIXME: confusing, is_before flipped
            if (is_before) {
                // six pts are clean
                let after_means1 =
                    Vec3::mean_from_three_vertices(six_pts[0], six_pts[1], six_pts[2]); //abc
                let after_means2 =
                    Vec3::mean_from_three_vertices(six_pts[3], six_pts[4], six_pts[5]); //def
                let after_normals1 =
                    Vec3::normal_from_three_vertices(six_pts[0], six_pts[1], six_pts[2]);
                let after_normals2 =
                    Vec3::normal_from_three_vertices(six_pts[3], six_pts[4], six_pts[5]);
                println!("t1,");
                csv_out_triangles([after_means1, after_means2],[after_normals1, after_normals2]);
                // --**--
                println!("delta,");
                println!(
                    "distance, {},",
                    - means[0].distance(means[1]) + after_means1.distance(after_means2)
                );
                let mut q = Quat::rot_between_vecs(normals[0], after_normals1);
                println!("rotation,");
                println!("abc,");
                csv_out_rotation(q);
                println!("def,");
                q = Quat::rot_between_vecs(normals[1], after_normals2);
                csv_out_rotation(q);
                } else {
                means[0] = Vec3::mean_from_three_vertices(six_pts[0], six_pts[1], six_pts[2]); //abc
                means[1] = Vec3::mean_from_three_vertices(six_pts[3], six_pts[4], six_pts[5]); //def
                normals[0] = Vec3::normal_from_three_vertices(six_pts[0], six_pts[1], six_pts[2]);
                normals[1] = Vec3::normal_from_three_vertices(six_pts[3], six_pts[4], six_pts[5]);
                println!("t0");
                csv_out_triangles(means, normals);
            }
        }
    }
    Ok(())
}

fn csv_out_triangle(mean: Vec3, normal: Vec3) {
    println!("origin, {}, {}, {},", mean.x(), mean.y(), mean.z());
    println!("normal, {}, {}, {},", normal.x(), normal.y(), normal.z());
}

fn csv_out_triangles(means: [Vec3; 2], normals: [Vec3; 2]) {
    println!("abc,");
    csv_out_triangle(means[0], normals[0]);
    println!("def,");
    csv_out_triangle(means[1], normals[1]);
}

fn csv_out_rotation(q: Quat) {
    let mat = Mat3::from(q);
    let e = mat.eular_angle_zyz();
    let y = mat.yaw_pitch_roll();
    println!("eular, {}, {}, {},", e.0, e.1, e.2);
    println!("ypr, {}, {}, {},", y.0, y.1, y.2);
}

fn get_mean_coordinate(r: StringRecord) -> Option<Vec3> {
    let start = 3;

    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    for i in 0..5 {
        x += r.get(start + i * 3)?.parse::<f64>().unwrap();
        y += r.get(start + i * 3 + 1)?.parse::<f64>().unwrap();
        z += r.get(start + i * 3 + 2)?.parse::<f64>().unwrap();
    }

    let avg = Vec3::new(x, y, z).scalar_div(5.0);

    // println!("           avg: {}", avg);
    Some(avg)
}

fn main() {
    if let Err(error) = read_csv() {
        println!("error runing 'read_csv': {}", error);
        process::exit(1);
    }
}
