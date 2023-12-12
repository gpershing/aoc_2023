use crate::point::Point;

pub fn get_galaxy_coords(lines: &mut impl Iterator<Item=String>, expand: i64) -> Vec<Point> {
    let mut points = Vec::new();
    let mut lines = lines.peekable();
    let first_line = lines.peek().unwrap();
    let mut empty_cols: Vec<_> = std::iter::repeat(true).take(first_line.len()).collect();

    let mut y = 0;
    lines.for_each(|line| {
        let mut any_galaxies = false;
        line.char_indices().for_each(|(index, c)| {
            match c {
                '#' => {
                    any_galaxies = true;
                    empty_cols[index] = false;
                    points.push(Point::new(index as i64, y));
                },
                _ => ()
            }
        });
        y += if any_galaxies { 1 } else { expand } 
    });

    let mut x_coord_map: Vec<_> = (0..empty_cols.len()).map(|x| x as i64).collect();
    let mut offset = 0;
    x_coord_map.iter_mut().enumerate().for_each(|(index, x)| {
        *x += offset;
        if empty_cols[index] {
            offset += expand - 1;
        }
    });

    points.iter_mut().for_each(|point| *point = Point::new(x_coord_map[point.x as usize], point.y));
    points
}