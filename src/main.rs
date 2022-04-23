extern crate cairo;

use clap::Parser;
use std::fs::File;

use colorsys::{Rgb};
use cairo::{ ImageSurface, Format, Context };

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Stroke {

    #[clap(short, long, default_value = "00ffff")]
    color: String,


    #[clap(short, long, default_value = "output.png")]
    output: String,

    to: Vec<f64>,
}

trait CairoStroke {
    fn draw(&self) -> ();
}


impl CairoStroke for Stroke {

    fn draw(&self) -> () {

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

        if points.len() < 2 {
            panic!("points is too less");
        }

        let color = Rgb::from_hex_str(&self.color).unwrap();

        let canvas = ImageSurface::create(Format::ARgb32, size[0] as i32, size[1] as i32).expect("Failed to create surface");
        let context = Context::new(&canvas).expect("Failed to create surface");

        context.set_source_rgb(color.red(), color.green(), color.blue());
        for i in 0..points.len() {
            context.line_to(points[i][0], points[i][1]);
        }
        context.stroke();
        let mut file = File::create(&self.output).expect("Failed to open output png");
        canvas.write_to_png(&mut file);

        println!("✨ Done!\n\nYou can just run:\n  open {}", self.output);
    }
}

fn main() {
    let stk = Stroke::parse();
    stk.draw();
}

