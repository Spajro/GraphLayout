use image::{Rgba, RgbaImage};
use imageproc::drawing::*;
use rusttype::{Font, Scale};
use crate::force_driven_layout::State;
use crate::math2d::{add, NormalizedVector, scale_position_x, scale_position_y};

static WIDTH: u32 = 1920;
static HEIGHT: u32 = 1080;

pub fn draw(state: &mut State) {
    state.graph.vertexes.iter().for_each(|vertex|
        println!("ADJUST AT {},{}",
                 state.positions.get(&vertex).unwrap().x,
                 state.positions.get(&vertex).unwrap().y));
    adjust(state);
    let mut image = RgbaImage::new(WIDTH, HEIGHT);
    let data = std::fs::read("resources/Millenia.ttf").unwrap();
    let font = &Font::try_from_vec(data).unwrap_or_else(|| {
        panic!();
    });
    state.graph.vertexes.iter().for_each(|vertex| {
        println!("DRAW AT {},{}",
                 state.positions.get(&vertex).unwrap().x,
                 state.positions.get(&vertex).unwrap().y);
        image = draw_text::<RgbaImage>(&image,
                                       Rgba([206, 127, 223, 16]),
                                       state.positions.get(&vertex).unwrap().x/2,
                                       state.positions.get(&vertex).unwrap().y/2,
                                       Scale { x: 30.0, y: 30.0 },
                                       font,
                                       vertex.label.as_str());});
    let _ = image.save("result.jpg");
}

fn adjust(state: &mut State) {
    adjust_to_non_negative(state);
    adjust_scale(state);
}

fn adjust_to_non_negative(state: &mut State) {
    let mut minx = 0;
    let mut miny = 0;
    state.positions.iter()
        .for_each(|pair| {
            if pair.1.x < minx {
                minx = pair.1.x;
            }
            if pair.1.y < miny {
                miny = pair.1.y;
            }
        });
    let movement = NormalizedVector {
        x: minx*(-1),
        y: miny*(-1),
    };
    state.graph.vertexes.iter()
        .for_each(|vertex| { state.positions.insert(vertex.clone(), add(state.positions.get(vertex).unwrap().clone(), movement)); });
}

fn adjust_scale(state: &mut State) {
    let mut maxx: i32 = WIDTH as i32;
    let mut maxy: i32 = HEIGHT as i32;
    state.positions.iter()
        .for_each(|pair| {
            if pair.1.x > maxx {
                maxx = pair.1.x;
            }
            if pair.1.y > maxy {
                maxy = pair.1.y;
            }
        });
    if maxx > WIDTH as i32 {
        let diff: f64 = (WIDTH as f64) / (maxx as f64);
        state.graph.vertexes.iter()
            .for_each(|vertex| { state.positions.insert(vertex.clone(), scale_position_x(state.positions.get(vertex).unwrap().clone(), diff)); });
    }
    if maxy > HEIGHT as i32 {
        let diff: f64 = (HEIGHT as f64) / (maxy as f64);
        state.graph.vertexes.iter()
            .for_each(|vertex| { state.positions.insert(vertex.clone(), scale_position_y(state.positions.get(vertex).unwrap().clone(), diff)); });
    }
}