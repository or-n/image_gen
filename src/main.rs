use std::time::Instant;

use num::{
    constant::{max_value::*, zero::*},
    interpolate::*,
    operation::{decrement::*, increment::*, length::LengthSquared},
    point::{bounds::*, _2::*, _3::*},
    ratio::f32::*,
    scale::*,
};

use image::{ImageBuffer, Rgb};

const SIZE: _2<u32> = _2([512, 512]);

fn with_corners(values: [_3<u8>; 4]) -> impl Fn(u32, u32) -> Rgb<u8> {
    move |x, y| {
        let ratio = _2([x, y]).zip(SIZE.decrement(), f32_ratio);
        let color = ratio.0.interpolate(&values);
        Rgb(color.0)
    }
}

fn with_corners_brightness_corrected(values: [_3<u8>; 4]) -> impl Fn(u32, u32) -> Rgb<u8> {
    move |x, y| {
        let ratio = _2([x, y]).zip(SIZE.decrement(), f32_ratio);
        let real_values = values.map(|color| {
            let f = |x| (x as u16) * (x as u16);
            _3(color.0.map(f))
        });
        let color = ratio.0.interpolate(&real_values);
        let back = color.0.map(|x| (x as f32).sqrt() as u8);
        Rgb(back)
    }
}

fn circle(r: u32, inside: _3<u8>, outside: _3<u8>) -> impl Fn(u32, u32) -> Rgb<u8> {
    move |x, y| {
        let r = 2 * r as i32;
        let p: _2<i32> = _2([x, y]).scale(2).increment().into();
        let color = if (p - SIZE.into()).length_squared() <= r * r {
            inside
        } else {
            outside
        };
        Rgb(color.0)
    }
}

fn save_image(name: &str, f: impl Fn(u32, u32) -> Rgb<u8>) {
    ImageBuffer::from_fn(SIZE.0[0], SIZE.0[1], f)
        .save(name.to_owned() + ".png")
        .unwrap();
}

fn main() {
    let start_time = Instant::now();
    let black = _3::zero();
    let white = _3::max_value();
    let bounds = Bounds::max_value();
    let [r, g, b] = _3::all_axis(bounds);
    let y = _3::axis2(-bounds);
    save_image("circle", circle(SIZE.0[0] / 2, r, black));
    save_image("rybg", with_corners([r, y, b, g]));
    save_image("r01g", with_corners([r, black, white, g]));
    save_image("0yb1", with_corners([black, y, b, white]));
    save_image(
        "rybg_corrected",
        with_corners_brightness_corrected([r, y, b, g]),
    );
    save_image(
        "r01g_corrected",
        with_corners_brightness_corrected([r, black, white, g]),
    );
    save_image(
        "0yb1_corrected",
        with_corners_brightness_corrected([black, y, b, white]),
    );
    let end_time = Instant::now();
    println!("Done {:?}", end_time - start_time);
}
