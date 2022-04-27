mod canvas;
use skia_safe::{Color};
use canvas::Canvas;

use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use colorsys::Rgb;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Stroke {

    #[clap(short, long, default_value = "00ffff")]
    color: String,


    #[clap(short, long, default_value = "output.png")]
    output: String,


    #[clap(short, long)]
    view: bool,

    to: Vec<f64>,
}

trait CairoStroke {
    fn get_points_and_size(&self) -> (Vec<[f64; 2]>, [f64; 2]);
    fn draw(&self, points: Vec<[f64; 2]>, size: [f64; 2]) -> ();
    fn open(&self) -> ();
}


impl CairoStroke for Stroke {

    fn get_points_and_size(&self) -> (Vec<[f64; 2]>, [f64; 2]) {
        let mut points: Vec<[f64; 2]> = Vec::new();
        let mut size: [f64; 2] = [0.0, 0.0];

        let mut counter = 0;
        loop {
            if counter + 2 <= self.to.len() {
                let x = self.to[counter];
                let y = self.to[counter + 1];
                if x > size[0] {
                    size[0] = x;
                }
                if y > size[1] {
                    size[1] = y;
                }
                points.push([x, y]);
                counter += 2;
                continue;
            }
            break;
        };
        (points, size)
    }

    fn draw(&self, points: Vec<[f64; 2]>, size: [f64; 2]) -> () {
        let mut canvas = Canvas::new(size[0] as i32, size[1] as i32);
        let context = canvas.get_context();

        let color = Rgb::from_hex_str(&self.color).expect("Failed to create color");

        context.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
        context.set_anti_alias(true);
        context.set_stroke_width(2.0);

        canvas.move_to(points[0][0] as f32, points[0][1] as f32);
        for i in 1..points.len() {
            canvas.line_to(points[i][0] as f32, points[i][1] as f32);
        }

        canvas.stroke();

        let d = canvas.data();

        let mut file = File::create(&self.output).expect("Failed to open output png");
        let bytes = d.as_bytes();

        file.write_all(bytes).expect("Failed to write output png");
    }

    fn open(&self) -> () {

        if self.view {
            println!("✨ Done!");
            Command::new("sh")
                .arg("-c")
                .arg("open ".to_string() + &self.output)
                .output()
                .expect("failed to execute process");
        } else {
            println!("✨ Done!\n\nYou can just run:\n  open {}", self.output);
        }
    }
}

fn main() {
    let stk = Stroke::parse();

    let (points, size) = stk.get_points_and_size();
    if points.len() < 2 {
        println!("\n🤔 Maybe you need run:\n  stroke -h");
        return;
    }
    stk.draw(points, size);
    stk.open();
}

