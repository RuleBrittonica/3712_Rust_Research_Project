use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Point {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug)]
pub struct PointDistance {
    pub distance_south: f64,
    pub latitude: f64,
    pub longitude: f64,
}

// Convert degrees to radians
fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * (PI / 180.0)
}

// Calculate the great-circle distance between two points
// on the Earth (specified in decimal degrees)
fn haversine(p1: &Point, p2: &Point) -> f64 {
    // Convert decimal degrees to radians
    let lat1 = degrees_to_radians(p1.lat);
    let lon1 = degrees_to_radians(p1.lng);
    let lat2 = degrees_to_radians(p2.lat);
    let lon2 = degrees_to_radians(p2.lng);

    // Haversine formula
    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * (a.sqrt().atan2((1.0 - a).sqrt()));

    // Earth radius in meters
    const R: f64 = 6371000.0;

    // Haversine distance
    return R * c
}

// Calculate distance south from the first point in the list of points
pub fn distance_south(points: &[Point]) -> Vec<PointDistance> {
    if points.is_empty() {
        return vec![];
    }

    let first_point = &points[0];
    let first_lat = first_point.lat;
    let first_lng = first_point.lng;

    let mut south_distances = Vec::new();

    for point in points {
        let lat = point.lat;
        let lon = point.lng;
        let distance_south_meters = haversine(first_point, &Point { lat, lng: first_lng });
        let distance_south_km = distance_south_meters / 1000.0;
        south_distances.push(PointDistance {
            distance_south: distance_south_km,
            latitude: lat,
            longitude: lon,
        });
    }

    south_distances
}

fn transpose_matrix(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let num_rows = matrix.len();
    let num_cols = matrix.iter().map(|row| row.len()).max().unwrap_or(0);

    let mut transposed = vec![vec![0; num_rows]; num_cols];

    for i in 0..num_rows {
        for j in 0..matrix[i].len() {
            transposed[j][i] = matrix[i][j];
        }
    }

    return transposed
}



// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degrees_to_radians() {
        let degrees = 180.0;
        let radians = degrees_to_radians(degrees);
        assert!((radians - PI).abs() < 1e-10, "Expected {} radians, got {}", PI, radians);

        let degrees = 0.0;
        let radians = degrees_to_radians(degrees);
        assert!((radians - 0.0).abs() < 1e-10, "Expected 0 radians, got {}", radians);

        let degrees = 360.0;
        let radians = degrees_to_radians(degrees);
        assert!((radians - 2.0 * PI).abs() < 1e-10, "Expected {} radians, got {}", 2.0 * PI, radians);
    }

    #[test]
    fn test_haversine() {
        let p1 = Point { lat: 52.2296756, lng: 21.0122287 };
        let p2 = Point { lat: 41.8919300, lng: 12.5113300 };

        let expected_distance = 1_316_000.0;

        let distance = haversine(&p1, &p2);

        assert!((distance - expected_distance).abs() < 1_000.0, "Expected distance around {} meters, got {}", expected_distance, distance);

        let p_same = Point { lat: 52.2296756, lng: 21.0122287 };
        let distance_same_point = haversine(&p1, &p_same);
        assert!(distance_same_point.abs() < 1e-10, "Expected distance around 0 meters, got {}", distance_same_point);

        let ny = Point { lat: 40.712776, lng: -74.005974 };
        let la = Point { lat: 34.052235, lng: -118.243683 };

        let expected_distance_ny_la = 3_936_000.0;

        let distance_ny_la = haversine(&ny, &la);

        assert!((distance_ny_la - expected_distance_ny_la).abs() < 1_000.0, "Expected distance around {} meters, got {}", expected_distance_ny_la, distance_ny_la);
    }

    #[test]
    fn test_distance_south() {
        let points = vec![
            Point { lat: 10.0, lng: 0.0 },
            Point { lat: 20.0, lng: 0.0 },
            Point { lat: 30.0, lng: 0.0 },
        ];

        let result = distance_south(&points);

        assert_eq!(result.len(), 3);

        assert!((result[0].distance_south - 0.0).abs() < 1e-10, "Expected distance around 0 km, got {}", result[0].distance_south);
        assert!((result[1].distance_south - 1111.0).abs() < 1.0, "Expected distance around 1111 km, got {}", result[1].distance_south);
        assert!((result[2].distance_south - 2224.0).abs() < 1.0, "Expected distance around 2222 km, got {}", result[2].distance_south);

        // Additional test with different longitudes
        let points_diff_lng = vec![
            Point { lat: 10.0, lng: 0.0 },
            Point { lat: 20.0, lng: 10.0 },
            Point { lat: 30.0, lng: 20.0 },
        ];

        let result_diff_lng = distance_south(&points_diff_lng);

        assert_eq!(result_diff_lng.len(), 3);

        // Test distances with different longitude values
        assert!((result_diff_lng[0].distance_south - 0.0).abs() < 1e-10, "Expected distance around 0 km, got {}", result_diff_lng[0].distance_south);
        assert!((result_diff_lng[1].distance_south - 1111.0).abs() < 1.0, "Expected distance around 1111 km, got {}", result_diff_lng[1].distance_south);
        assert!((result_diff_lng[2].distance_south - 2224.0).abs() < 1.0, "Expected distance around 2222 km, got {}", result_diff_lng[2].distance_south);
    }

    #[test]
    fn test_transpose_square_matrix() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        let expected: Vec<Vec<i32>> = vec![
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![3, 6, 9],
        ];

        assert_eq!(transpose_matrix(&matrix), expected);
    }

    #[test]
    fn test_transpose_rectangular_matrix() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];

        let expected: Vec<Vec<i32>> = vec![
            vec![1, 4],
            vec![2, 5],
            vec![3, 6],
        ];

        assert_eq!(transpose_matrix(&matrix), expected);
    }

    #[test]
    fn test_transpose_empty_matrix() {
        let matrix: Vec<Vec<i32>> = Vec::new();
        let expected: Vec<Vec<i32>> = Vec::new();

        assert_eq!(transpose_matrix(&matrix), expected);
    }

    #[test]
    fn test_transpose_single_element_matrix() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1],
        ];

        let expected: Vec<Vec<i32>> = vec![
            vec![1],
        ];

        assert_eq!(transpose_matrix(&matrix), expected);
    }

    #[test]
    fn test_transpose_empty_rows() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![],
            vec![],
        ];

        let expected: Vec<Vec<i32>> = vec![
            vec![],
            vec![],
        ];

        assert_eq!(transpose_matrix(&matrix), expected);
    }
}
