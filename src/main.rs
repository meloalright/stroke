/** 
 * Should not extern cairo
extern crate cairo;
 **/

use clap::Parser;
use std::fs::File;
use std::process::Command;

use colorsys::{Rgb};
use cairo::{ ImageSurface, Format, Context };

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

        let color = Rgb::from_hex_str(&self.color).unwrap();

        let canvas = ImageSurface::create(Format::ARgb32, size[0] as i32, size[1] as i32).expect("Failed to create surface");
        let context = Context::new(&canvas).expect("Failed to create context");

        context.set_source_rgb(color.red(), color.green(), color.blue());
        for i in 0..points.len() {
            context.line_to(points[i][0], points[i][1]);
        }
        context.stroke();
        let mut file = File::create(&self.output).expect("Failed to open output png");
        canvas.write_to_png(&mut file);
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

