use image::{Rgba, RgbaImage};
use imageproc::drawing::*;
use rusttype::{Font, Scale};
use crate::force_driven_layout::State;
use crate::math2d::{add, NormalizedVector, scale_position_x, scale_position_y};

static WIDTH: u32 = 3840;
static HEIGHT: u32 = 2160;
static DRAW_WIDTH: u32 = WIDTH - 200;
static DRAW_HEIGHT: u32 = HEIGHT - 200;

pub fn draw(state: &mut State) {
    let mut image = RgbaImage::new(WIDTH, HEIGHT);
    let data = std::fs::read("resources/GloriousFree.ttf").unwrap();
    let font = &Font::try_from_vec(data).unwrap_or_else(|| {
        panic!();
    });
    adjust(state);
    let mut i = 1;
    state.graph.vertexes.iter().for_each(|vertex| {
        println!("[{}/{}] DRAW VERTEX {} AT {},{}",
                 i,
                 state.graph.vertexes.len(),
                 vertex.label,
                 state.positions.get(&vertex).unwrap().x,
                 state.positions.get(&vertex).unwrap().y);
        i += 1;
        image = draw_text::<RgbaImage>(&image,
                                       Rgba([206, 127, 223, 200]),
                                       state.positions.get(&vertex).unwrap().x,
                                       state.positions.get(&vertex).unwrap().y,
                                       Scale { x: 30.0, y: 30.0 },
                                       font,
                                       vertex.label.as_str());
    });
    i = 1;
    state.graph.edges.iter().for_each(|edge| {
        println!("[{}/{}] DRAW EDGE {} -> {} AT {},{} => {},{}",
                 i,
                 state.graph.edges.len(),
                 edge.first.label,
                 edge.second.label,
                 state.positions.get(&edge.first).unwrap().x,
                 state.positions.get(&edge.first).unwrap().y,
                 state.positions.get(&edge.second).unwrap().x,
                 state.positions.get(&edge.second).unwrap().y);
        i += 1;
        image = draw_line_segment(&image,
                                  (state.positions.get(&edge.first).unwrap().x as f32, state.positions.get(&edge.first).unwrap().y as f32),
                                  (state.positions.get(&edge.second).unwrap().x as f32, state.positions.get(&edge.second).unwrap().y as f32),
                                  Rgba([60, 252, 91, 200]));
    });
    let _ = image.save("result.png");
}

pub(crate) fn adjust(state: &mut State) {
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
        x: minx * (-1),
        y: miny * (-1),
    };
    state.graph.vertexes.iter()
        .for_each(|vertex| { state.positions.insert(vertex.clone(), add(state.positions.get(vertex).unwrap().clone(), movement)); });
}

fn adjust_scale(state: &mut State) {
    let mut maxx: i32 = DRAW_WIDTH as i32;
    let mut maxy: i32 = DRAW_HEIGHT as i32;
    let final_movement = NormalizedVector {
        x: ((WIDTH - DRAW_WIDTH) / 2) as i32,
        y: ((HEIGHT - DRAW_HEIGHT) / 2) as i32,
    };
    state.positions.iter()
        .for_each(|pair| {
            if pair.1.x > maxx {
                maxx = pair.1.x;
            }
            if pair.1.y > maxy {
                maxy = pair.1.y;
            }
        });
    if maxx > DRAW_WIDTH as i32 {
        let diff: f64 = (DRAW_WIDTH as f64) / (maxx as f64);
        state.graph.vertexes.iter()
            .for_each(|vertex| {
                state.positions.insert(
                    vertex.clone(),
                    scale_position_x(state.positions.get(vertex).unwrap().clone(), diff));
            });
    }
    if maxy > DRAW_HEIGHT as i32 {
        let diff: f64 = (DRAW_HEIGHT as f64) / (maxy as f64);
        state.graph.vertexes.iter()
            .for_each(|vertex| {
                state.positions.insert(
                    vertex.clone(),
                    scale_position_y(state.positions.get(vertex).unwrap().clone(), diff));
            });
    }
    state.graph.vertexes.iter()
        .for_each(|vertex| {
            state.positions.insert(
                vertex.clone(),
                add(state.positions.get(vertex).unwrap().clone(), final_movement));
        });
}