use imageproc::drawing::*;
use crate::force_driven_layout::State;
use crate::math2d::{add, NormalizedVector, scale_position_x, scale_position_y};

static WIDTH: i32 = 1920;
static HEIGHT: i32 = 1080;

pub fn draw(state: &mut State) {
    adjust(state)
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
        x: minx,
        y: miny,
    };
    state.graph.vertexes.iter()
        .for_each(|vertex| { state.positions.insert(vertex.clone(), add(state.positions.get(vertex).unwrap().clone(), movement)); });
}

fn adjust_scale(state: &mut State) {
    let mut maxx = WIDTH;
    let mut maxy = HEIGHT;
    state.positions.iter()
        .for_each(|pair| {
            if pair.1.x > maxx {
                maxx = pair.1.x;
            }
            if pair.1.y > maxy {
                maxy = pair.1.y;
            }
        });
    if maxx > WIDTH {
        let diff: f64 = (WIDTH as f64) / (maxx as f64);
        state.graph.vertexes.iter()
            .for_each(|vertex| { state.positions.insert(vertex.clone(), scale_position_x(state.positions.get(vertex).unwrap().clone(), diff)); });
    }
    if maxy > HEIGHT {
        let diff: f64 = (HEIGHT as f64) / (maxy as f64);
        state.graph.vertexes.iter()
            .for_each(|vertex| { state.positions.insert(vertex.clone(), scale_position_y(state.positions.get(vertex).unwrap().clone(), diff)); });
    }
}