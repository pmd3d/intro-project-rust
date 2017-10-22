const PI: f64 = 3.141592;
const MAX_SECTOR: i16 = 36;
const MAX_CYLINDER: i16 = 5;
const MAX_RADIUS: i16 = 300;
const RADIUS_PER_CYLINDER: f64 = (MAX_RADIUS / MAX_CYLINDER) as f64;
pub const CENTER_X: i16 = 400;
pub const CENTER_Y: i16 = 300;

pub fn cylinders_to_triangles() -> Vec<Vec<i16>> {
    let mut points: Vec<Vec<i16>> = Vec::new();

    for cylinder in 0..MAX_CYLINDER {
        let mut circle: Vec<i16> = Vec::new();
        let mut angle: f64 = 0.0;
        for _sector in 0..MAX_SECTOR {
            let hypotenuse: f64 = RADIUS_PER_CYLINDER * cylinder as f64;
            let xoffset: i16 = (hypotenuse * angle.cos()) as i16;
            let yoffset: i16 = (hypotenuse * angle.sin()) as i16;
            let x = CENTER_X + xoffset;
            let y = CENTER_Y + yoffset;
            circle.push(x);
            circle.push(y);
            angle += PI / MAX_SECTOR as f64;
        }
        points.push(circle);
    }

    points
}
