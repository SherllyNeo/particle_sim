use raylib::prelude::*;
use raylib::color::Color;
use anyhow::Result;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::{self, f32};
use derivative::Derivative; // 2.2.0



#[derive(Debug, Eq, Derivative,Clone,Copy)]
#[derivative(PartialEq, Hash)]
enum Colour {
    RED,
    GREEN,
    BLUE,
    WHITE,
    ORANGE,
    PURPLE,
    YELLOW,
    PLUM,
    CORAL,
    FUCHSIA,
    NAVY,
    LAVENDERBLUSH
}


fn colour_to_color(c: &Colour) -> Color {
    match c {
        Colour::RED => Color::RED,
        Colour::GREEN => Color::GREEN,
        Colour::BLUE => Color::BLUE,
        Colour::WHITE => Color::WHITE,
        Colour::YELLOW => Color::YELLOW,
        Colour::ORANGE => Color::ORANGE,
        Colour::PURPLE => Color::PURPLE,
        Colour::PLUM => Color::PLUM,
        Colour::CORAL => Color::CORAL,
        Colour::FUCHSIA => Color::FUCHSIA,
        Colour::NAVY => Color::NAVY,
        Colour::LAVENDERBLUSH => Color::LAVENDERBLUSH 
    }
}



#[derive(Debug, Derivative,Clone, PartialEq)]
struct Particle {
    x: f64,
    y: f64,
    xv: f64,
    yv: f64,
    mass: f64,
    colour: Colour
}


impl Particle {
    fn new(screen_height: f64, screen_width: f64,colour: Colour) -> Particle {
        let mut rng = thread_rng();
        let return_particle = Particle {
            x: rng.gen_range(
                   0..(screen_width/(4 as f64) ) as i32
                   ) as f64 + (screen_width/(2 as f64)),
            y: rng.gen_range(
                   0..(screen_height/(4 as f64) ) as i32
                ) as f64 + (screen_height/(2 as f64)),
            xv: 0.0,
            yv: 0.0,
            mass: 1.0,
            colour
        };
        return_particle
    }
    fn draw(&self,drawer: &mut RaylibDrawHandle) {
        let color = colour_to_color(&self.colour);
        drawer.draw_circle(self.x as i32, self.y as i32 , (self.mass * 4 as f64) as f32, color);
    }
}

fn draw_all_particles(particles: &Vec<Particle>,drawer: &mut RaylibDrawHandle) {
    for p in particles {
        p.draw(drawer);
    }
}


fn update_particles(
    particles1: &mut [Particle],
    particles2: Option<&[Particle]>,
    screen_height: f64,
    screen_width: f64,
    min_distance: f64,
    force_distance: f64,
    gravity: f64,
    friction: f64,
    velocity_factor: f64,
) {

    for i in 0..particles1.len() {
        let mut particle1 = particles1[i].clone();
        let mut fx = 0.0;
        let mut fy = 0.0;

        for j in 0..particles2.unwrap_or(particles1).len() {
            let particle2 = &particles2.unwrap_or(particles1)[j];
            let dx = particle1.x - particle2.x;
            let dy = particle1.y - particle2.y;
            let distance = (dx * dx + dy * dy).sqrt();
            let mut f = 0.0;
            if distance > force_distance {
                continue;
            }

            if distance < min_distance {
                f = -gravity * particle1.mass * particle2.mass;
            }

            if distance > min_distance && distance < force_distance {
                f = gravity * particle1.mass * particle2.mass / distance;
            }

            fx += (f - particle1.xv * friction) * dx;
            fy += (f - particle1.yv * friction) * dy;
            particle1.xv = (particle1.xv + fx) * velocity_factor;
            particle1.yv = (particle1.yv + fy) * velocity_factor;
        }


        particle1.x += particle1.xv;
        particle1.y += particle1.yv;

        // Wrapping around the screen boundaries
        particle1.x = (particle1.x + screen_width) % screen_width;
        particle1.y = (particle1.y + screen_height) % screen_height;

        particles1[i] = particle1;
    }

}

#[derive(Clone,Copy)]
struct Value {
    colour: Colour,
    amount: i32
}


fn main() -> Result<()> {
    let amount = 500;

    // Read config file for below info
    let values = [
        Value {colour: Colour::NAVY,  amount },
        Value {colour: Colour::LAVENDERBLUSH,  amount },
        Value {colour: Colour::FUCHSIA, amount },
        Value {colour: Colour::PLUM,  amount },
        Value {colour: Colour::CORAL,  amount },
    ];

    // For each color, make a random value for their intereactions
    // Make a new vector with the values
    // Calculate interactions between each
    // Draw each

    
    let screen_height = 1080;
    let screen_width = 1920;
    let radius =  5.0;
    let velocity_factor =  0.1;
    let friction =  0.0;
    let min_distance =  1.0;
    let force_distance =  radius*(screen_width as f64)*(screen_height as f64)*0.00004;

    let (mut rl,thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Particle life")
        .build();
    rl.set_target_fps(60);
    let mut rng = rand::thread_rng();

    let mut colors_map:  HashMap<(Colour, Colour), f64> = HashMap::new();

    let mut all_colours: Vec<Vec<Particle>> = Vec::new();
    for value in values.clone() {
        let new_particles: Vec<Particle> = (0..=value.amount).map(|_| Particle::new(screen_height as f64,screen_width as f64,value.colour)).collect::<Vec<Particle>>();
        all_colours.push(new_particles);
        for value2 in values {
            let key = (value.colour,value2.colour);
            let force = rng.gen_range(-1.0..1.0);
            colors_map.insert(key,force.into());

        }
    }

    println!("{:?}",colors_map);




    while !rl.window_should_close() {
        // update
       // let colour_force = gravity as f64*0.8;
       // update_particles(&mut red_particles, Some(&blue_particles), screen_height as f64, screen_width as f64, mindistance as f64, forcedistance as f64, colour_force, friction, velocityfactor);
       // let colour_force = gravity as f64*-0.5;
       // update_particles(&mut blue_particles, Some(&ed_particles), screen_height as f64, screen_width as f64, mindistance as f64, forcedistance as f64, colour_force, friction, velocityfactor);
       // let colour_force = gravity as f64*-0.1;
       // update_particles(&mut blue_particles, None, screen_height as f64, screen_width as f64, mindistance as f64, forcedistance as f64, colour_force, friction, velocityfactor);


        let length = all_colours.len();
        for i in 0..length {
            for j in 0..length {
                let colour = all_colours[i][0].colour;
                let colour2 = all_colours[j][0].colour;

                let mut colour_vec2 = None;
                let clone = all_colours[j].clone();
                if i != j {
                    colour_vec2 = Some(clone.as_slice());
                }

                let mut colour_vec = &mut all_colours[i];

                let gravity = *colors_map.get(&(colour,colour2)).unwrap();

                update_particles(&mut colour_vec, colour_vec2, screen_height as f64, screen_width as f64, min_distance, force_distance, gravity, friction, velocity_factor);
            }
        }



        // draw
        let mut drawer = rl.begin_drawing(&thread);
        drawer.clear_background(Color::BLACK);

        for colour_vec in &all_colours {
            draw_all_particles(&colour_vec,&mut drawer);
        }
    }

    Ok(())
}
