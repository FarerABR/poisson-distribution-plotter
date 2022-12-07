use plotters::prelude::*;
use std::{env, f32::consts};

/**
## config for IO arguments

```
param: [String]

returns: miu:i32, vrc:i32, perc:i32, dom:i32
```
*/
fn config(args: &[String]) -> u32 {
    if args.len() < 1 {
        panic!("not enough arguments");
    }
    args[1].parse::<f32>().unwrap() as u32
}

/**
## fact funtion
```
param: n:u128
returns: u128
```
 */
fn fact(n: u128) -> u128 {
    match n {
        0 => 1,
        _ => fact(n - 1) * n,
    }
}

/**
## poisson distribution function
```
param: a:i32 , k:u32
returns: f32
```
 */
fn poisson(a: i32, k: u32) -> f32 {
    consts::E.powi(-a) * (a.pow(k) as f32 / fact(k as u128) as f32) as f32
}

fn draw(a: u32) {
    let mut y: Vec<(f32, f32)> = Vec::new();

    for i in 0..=100 {
        let temp = poisson(a as i32, i);
        if temp < 0.0001 {
            break;
        }
        y.push((i as f32, temp));
    }

    let drawing_area = BitMapBackend::new("./figure.png", (1280, 1706)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut max_n = 0.0;
    for i in &y {
        if max_n < i.1 {
            max_n = i.1;
        }
    }
    let mut chart = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(format!("poisson distribution - p({})", a), ("mono", 40))
        .build_cartesian_2d(0.0..y[y.len() - 1].0 as f32 + 1.0, 0.0..max_n + 0.01)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            y.iter().map(|&point| {
                TriangleMarker::new(point, 5, Into::<ShapeStyle>::into(&RED).filled())
            }),
        )
        .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let a = config(&args);
    draw(a);
}
