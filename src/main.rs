extern crate embedded_graphics as graphics;

use graphics::Drawing;
use graphics::drawable::Pixel;
use graphics::primitives::Line;

struct Basic {}

impl Drawing for Basic {
    fn draw<T>(&mut self, item_pixels: T)
    where T: Iterator<Item = Pixel> {
        for pixel in item_pixels {
            print!("{} {};", (pixel.0).0, (pixel.0).1);
        }
        print!("\n");
    }
}

fn main() {
    let mut tester = Basic {};

    let points = [ 
        (125, 65), (124, 71), (123, 77), (122, 83), (119, 89),
        (116, 94), (113, 100), (109, 105), (105, 109), (100, 113),
        (95, 116), (89, 119), (83, 122), (77, 123), (71, 124), 

        (65, 125), (59, 124), (53, 123), (47, 122), (41, 119), 
        (36, 116), (30, 113), (25, 109), (21, 105), (17, 100),
        (14, 95), (11, 89), (8, 83), (7, 77), (6, 71),

        (5, 65), (6, 59), (7, 53), (8, 47), (11, 41), 
        (14, 36), (17, 30), (21, 25), (25, 21), (30, 17),
        (35, 14), (41, 11), (47, 8), (53, 7), (59, 6),

        (65, 5), (71, 6), (77, 7), (83, 8), (89, 11),
        (94, 14), (100, 17), (105, 21), (109, 25), (113, 30),
        (116, 35), (119, 41), (122, 47), (123, 53), (124, 59)
    ];

    for point in points.iter() {
        print!("{} {};", point.0, point.1);
        tester.draw(Line::new( (65, 65), *point, 1 ).into_iter());
    }
}
