pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let parsed_points = Self::parse_points(points);

        let sorted_points = Self::sort_points(parsed_points);

        return sorted_points[0..k as usize].to_vec();
    }

    fn parse_points(points: Vec<Vec<i32>>) -> Vec<PointDistance> {
        let mut point_distance_vec = Vec::with_capacity(points.len());

        for point in points {
            let point_coord = Point::from(point);
            let point_distance = PointDistance::new(point_coord);

            point_distance_vec.push(point_distance);
        }

        return point_distance_vec;
    }

    fn sort_points(mut point_distances: Vec<PointDistance>) -> Vec<Vec<i32>> {
        let point_distances_len = point_distances.len();

        for idx_i in 0..point_distances_len {
            for idx_j in idx_i..point_distances_len {
                if point_distances[idx_j].distance < point_distances[idx_i].distance {
                    let temp_point_distance = point_distances[idx_j];
                    point_distances[idx_j] = point_distances[idx_i];
                    point_distances[idx_i] = temp_point_distance;
                }
            }
        }

        let mut point_vec = Vec::with_capacity(point_distances_len);

        for point_distance in point_distances.iter() {
            point_vec.push(point_distance.to_vec_int());
        }
        return point_vec;
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl From<Vec<i32>> for Point {
    fn from(point_vec: Vec<i32>) -> Self {
        if point_vec.len() < 2 {
            panic!("Can't be casted to Point");
        }

        Self {
            x: point_vec[0],
            y: point_vec[1],
        }
    }
}

#[derive(Clone, Copy)]
struct PointDistance {
    point: Point,
    distance: f32,
}

impl PointDistance {
    fn new(point: Point) -> Self {
        let x_squared = (point.x - 0).pow(2);
        let y_squared = (point.y - 0).pow(2);

        let distance = ((x_squared + y_squared) as f32).sqrt();

        Self { point, distance }
    }

    fn to_vec_int(&self) -> Vec<i32> {
        vec![self.point.x, self.point.y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_closest() {
        let point_vec = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];

        let closest_points = Solution::k_closest(point_vec, 2);

        assert_eq!(closest_points, vec![vec![3, 3], vec![-2, 4]]);
    }
}
