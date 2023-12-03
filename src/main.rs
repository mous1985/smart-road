
extern crate sdl2;
extern crate rand;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
//use sdl2::Sdl;
//use sdl2::VideoSubsystem;
use sdl2::render::Canvas;
use sdl2::video::Window;
//use sdl2::render::RenderTarget;

use sdl2::rect::Rect;
use std::time::Duration;
use std::time::Instant;
//use std::env::set_var;
//use std::time::{Duration, Instant};
pub mod cars;
//use crate::cars::Coordonnees;
use crate::cars::Rectangle;
use crate::cars::Car;
//use crate::cars::cars;
use crate::cars::pointille_width;
use crate::cars::pointille_length;
use crate::cars::voie_width;
//use crate::cars::voie_length;
use crate::cars::road_width;
//use crate::cars::road_length;
//use crate::cars::carrefour_start;
//use crate::cars::carrefour_end;
//use crate::cars::car_width;
use crate::cars::car_length;
pub mod road;
use crate::road::Direction;
use crate::road::Voie;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
const WINDOW_WIDTH: i32 = 1000;
const WINDOW_HEIGHT: i32 = 1000;

/*fn generate_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..=3);
    number
}*/

//pub static mut cars: Vec<Car> = Vec::<Car>::new();

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let roads = vec![
        Rectangle::new(500, 0, road_width, 1000),
        Rectangle::new(500-road_width, 0, road_width, 1000),
        Rectangle::new(0, 500, 1000, road_width),
        Rectangle::new(0, 500-road_width, 1000, road_width),
    ];
    let tracage_voie_ext_int = vec![
        //centre up
        Rectangle::new(500, 0, pointille_width, 500-(3*voie_width+3*pointille_width)),
        Rectangle::new(500, 500+(3*voie_width+3*pointille_width), pointille_width, 500-(3*voie_width+3*pointille_width)),
        ////ext up
        Rectangle::new(500+(3*voie_width+3*pointille_width), 0, pointille_width, 500-(3*voie_width+3*pointille_width)),
        Rectangle::new(500+(3*voie_width+3*pointille_width), 500+(3*voie_width+3*pointille_width), pointille_width, 500-(3*voie_width+3*pointille_width)),
        //centre down
        Rectangle::new(500-pointille_width, 0, pointille_width, 500-(3*voie_width+3*pointille_width)),
        Rectangle::new(500-pointille_width, 500+(3*voie_width+3*pointille_width), pointille_width, 500-(3*voie_width+3*pointille_width)),
        ////ext down
        Rectangle::new(500-(3*voie_width+4*pointille_width), 0, pointille_width, 500-(3*voie_width+3*pointille_width)),
        Rectangle::new(500-(3*voie_width+4*pointille_width), 500+(3*voie_width+3*pointille_width), pointille_width, 500-(3*voie_width+3*pointille_width)),
        //centre right
        Rectangle::new(0, 500, 500-(3*voie_width+3*pointille_width), pointille_width),
        Rectangle::new(500+(3*voie_width+3*pointille_width), 500, 500-(3*voie_width+3*pointille_width), pointille_width),
        //ext right
        Rectangle::new(0, 500+(3*voie_width+3*pointille_width), 500-(3*voie_width+3*pointille_width), pointille_width),
        Rectangle::new(500+(3*voie_width+3*pointille_width), 500+(3*voie_width+3*pointille_width), 500-(3*voie_width+3*pointille_width), pointille_width),
        //centre left
        Rectangle::new(0, 500-pointille_width, 500-(3*voie_width+3*pointille_width), pointille_width),
        Rectangle::new(500+(3*voie_width+3*pointille_width), 500-pointille_width, 500-(3*voie_width+3*pointille_width), pointille_width),
        //ext left
        Rectangle::new(0, 500-(3*voie_width+4*pointille_width), 500-(3*voie_width+3*pointille_width), pointille_width),
        Rectangle::new(500+(3*voie_width+3*pointille_width), 500-(3*voie_width+4*pointille_width), 500-(3*voie_width+3*pointille_width), pointille_width),
        
        Rectangle::new(500-2*pointille_width, 500-2*pointille_width, 4*pointille_width, 4*pointille_width),
    ];
    let mut tracage_separation_voie = Vec::new();
    let mut i: i32 = 20;
    loop {
        tracage_separation_voie.push(Rectangle::new(500+voie_width+pointille_width, i, pointille_width, pointille_length));
        tracage_separation_voie.push(Rectangle::new(500+2*voie_width+2*pointille_width, i, pointille_width, pointille_length));
        tracage_separation_voie.push(Rectangle::new(500-voie_width-2*pointille_width, i, pointille_width, pointille_length));
        tracage_separation_voie.push(Rectangle::new(500-2*voie_width-3*pointille_width, i, pointille_width, pointille_length));
        tracage_separation_voie.push(Rectangle::new(i, 500+voie_width+pointille_width, pointille_length, pointille_width));
        tracage_separation_voie.push(Rectangle::new(i, 500+2*voie_width+2*pointille_width, pointille_length, pointille_width));
        tracage_separation_voie.push(Rectangle::new(i, 500-voie_width-2*pointille_width, pointille_length, pointille_width));
        tracage_separation_voie.push(Rectangle::new(i, 500-2*voie_width-3*pointille_width, pointille_length, pointille_width));
        i += pointille_length*2;
        if i > 1000 {
            break;
        }
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("SDL2 Example", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut running = true;
    
    let mut car_number: i32 = 0;
    let mut cars_priority = Vec::<Car>::new();
    let mut cars_passed_intersection = Vec::<Car>::new();
    let mut now = Instant::now();
    //set_var("RUST_BACKTRACE", "1");
    while running {
        //std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        let elapsed_time = now.elapsed();
        if elapsed_time > Duration::from_millis(16) {
            //println!("{} fps, {} car number, {} usize", 1000 / elapsed_time.as_millis(), cars_priority.len(), cars_priority.len() as usize);
            now = Instant::now();

            // event handling
            let mut event_number = 0;
            for event in sdl_context.event_pump().unwrap().poll_iter() {
                event_number += 1;
                let voi = Voie::rand();
                //let voi = Voie::Centre;
                // read all events from window
                //let s = String::from("aaaaa");
                match event {
                    Event::Quit { .. } => {
                        running = false;
                        break;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        running = false;
                        break;
                    }
                    Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => {
                        if cars_priority.len() < 200 {
                            //unsafe { cars.push(Car::new(Direction::Up)); };
                            if voi == Voie::Droite {
                                let i = cars_last_position(&cars_passed_intersection, Direction::Up, voi);
                                let new_car = Car::new(Direction::Up, voi, i, car_number);
                                cars_passed_intersection.push(new_car);
                            } else {
                                let i = cars_last_position(&cars_priority, Direction::Up, voi);
                                let new_car = Car::new(Direction::Up, voi, i, car_number);
                                //cars_priority = cars_insert_by_priority(cars_priority, new_car);
                                cars_priority.push(new_car);
                                println!("{}", new_car.number);
                            }
                            car_number += 1;
                            //cars.push(a_car);
                            //cars.push(Car::new(Direction::Up, voi, i));
                            //break;
                        } 
                    }
                    Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => { 
                        if cars_priority.len() < 200 {
                            //unsafe { cars.push(Car::new(Direction::Down)); };
                            if voi == Voie::Droite {
                                let i = cars_last_position(&cars_passed_intersection, Direction::Down, voi);
                                let new_car = Car::new(Direction::Down, voi, i, car_number);
                                cars_passed_intersection.push(new_car);
                            } else {
                                let i = cars_last_position(&cars_priority, Direction::Down, voi);
                                let new_car = Car::new(Direction::Down, voi, i, car_number);
                                //cars_priority = cars_insert_by_priority(cars_priority, new_car);
                                cars_priority.push(new_car);
                                println!("{}", new_car.number);
                            }
                            car_number += 1;
                            //cars.push(a_car);
                            //cars.push(Car::new(Direction::Down, voi, i));
                            //break;
                        }
                    }
                    Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => { 
                        if cars_priority.len() < 200 {
                            if voi == Voie::Droite {
                                //unsafe { cars.push(Car::new(Direction::Right)); };
                                let i = cars_last_position(&cars_passed_intersection, Direction::Right, voi);
                                let new_car = Car::new(Direction::Right, voi, i, car_number);
                                cars_passed_intersection.push(new_car);
                            } else {
                                //unsafe { cars.push(Car::new(Direction::Right)); };
                                let i = cars_last_position(&cars_priority, Direction::Right, voi);
                                let new_car = Car::new(Direction::Right, voi, i, car_number);
                                //cars_priority = cars_insert_by_priority(cars_priority, new_car);
                                cars_priority.push(new_car);
                                println!("{}", new_car.number);
                            }
                            car_number += 1;
                            //cars.push(a_car);
                            //cars.push(Car::new(Direction::Right, voi, i));
                            //break;
                        }
                    }
                    Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => { 
                        if cars_priority.len() < 200 {
                            //unsafe { cars.push(Car::new(Direction::Left)); };
                            if voi == Voie::Droite {
                                let i = cars_last_position(&cars_passed_intersection, Direction::Left, voi);
                                let new_car = Car::new(Direction::Left, voi, i, car_number);
                                cars_passed_intersection.push(new_car);
                            } else {
                                let i = cars_last_position(&cars_priority, Direction::Left, voi);
                                let new_car = Car::new(Direction::Left, voi, i, car_number);
                                //cars_priority = cars_insert_by_priority(cars_priority, new_car);
                                cars_priority.push(new_car);
                                println!("{}, {}", new_car.number, cars_priority.len());
                            }
                            car_number += 1;
                            //cars.push(a_car);
                            //cars.push(Car::new(Direction::Left, voi, i));
                            //break;
                        }
                    }
                    _ => {}
                }
            }
            //println!("{} car number, {} event number", cars_priority.len(), event_number);

            // draw the screen
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(90, 90, 90));
            for a_road in &roads {
                canvas.fill_rect(Rect::new(a_road.position.x, a_road.position.y, a_road.width as u32, a_road.height as u32));
            };
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            for a_voie_delimitation in &tracage_voie_ext_int {
                canvas.fill_rect(Rect::new(a_voie_delimitation.position.x, a_voie_delimitation.position.y, a_voie_delimitation.width as u32, a_voie_delimitation.height as u32));
            };
            for a_voie_separateur in &tracage_separation_voie {
                canvas.fill_rect(Rect::new(a_voie_separateur.position.x, a_voie_separateur.position.y, a_voie_separateur.width as u32, a_voie_separateur.height as u32));
            };
    
            //let (car_priority_returned, passed, a_passed_car) = cars_distribute_by_priority(cars_priority);
            (cars_priority, cars_passed_intersection) = cars_distribute_by_priority(cars_priority, cars_passed_intersection);
            //println!("{} car number, after priority", cars_priority.len());
            /*if passed {
                cars_passed_intersection.push(a_passed_car);
            }*/
    
            cars_priority = cars_to_screen(cars_priority, &mut canvas);
            cars_passed_intersection = cars_to_screen(cars_passed_intersection, &mut canvas);
            //println!("{} car number, after draw", cars_priority.len());
            //println!("");
            canvas.present();
            
        }
    }
}

pub fn cars_to_screen(car_arr: Vec<Car>, canvas: &mut Canvas<Window>) -> Vec<Car> {
    let mut running_cars = Vec::<Car>::new();
    //unsafe {
        for mut a_car in car_arr {
            if a_car.in_screen() {
                canvas.set_draw_color(a_car.color);
                canvas.fill_rect(Rect::new(a_car.hitbox.position.x, a_car.hitbox.position.y, a_car.hitbox.width as u32, a_car.hitbox.height as u32));
            }
            a_car.do_move();
            if !a_car.outside_screen() {
                running_cars.push(a_car);
            };
        };
    //};
    //unsafe { cars = running_cars; };
    return running_cars
}

//retourne la position de la dernière voiture dans la direction donnée
pub fn cars_last_position(car_arr: &Vec<Car>, dir: Direction, voi: Voie) -> i32 {
    for a_car in car_arr {
        if a_car.direction == dir && a_car.voie == voi {
            match dir {
                Direction::Up => {
                    let mut greater: bool = true;
                    for an_other_car in car_arr {
                        if an_other_car.direction == dir && an_other_car.voie == voi {
                            if a_car.hitbox.position.y < an_other_car.hitbox.position.y {
                                greater = false;
                            }
                        }
                    }
                    if greater {
                        return a_car.hitbox.position.y;
                    }
                },
                Direction::Down => {
                    let mut lower: bool = true;
                    for an_other_car in car_arr {
                        if an_other_car.direction == dir && an_other_car.voie == voi {
                            if a_car.hitbox.position.y > an_other_car.hitbox.position.y {
                                lower = false;
                            }
                        }
                    }
                    if lower {
                        return a_car.hitbox.position.y;
                    }
                },
                Direction::Right => {
                    let mut lower: bool = true;
                    for an_other_car in car_arr {
                        if an_other_car.direction == dir && an_other_car.voie == voi {
                            if a_car.hitbox.position.x > an_other_car.hitbox.position.x {
                                lower = false;
                            }
                        }
                    }
                    if lower {
                        return a_car.hitbox.position.x;
                    }
                },
                Direction::Left => {
                    let mut greater: bool = true;
                    for an_other_car in car_arr {
                        if an_other_car.direction == dir && an_other_car.voie == voi {
                            if a_car.hitbox.position.x < an_other_car.hitbox.position.x {
                                greater = false;
                            }
                        }
                    }
                    if greater {
                        return a_car.hitbox.position.x;
                    }
                },
            }
        }
    }
    return 0
}

// trier les vehicules en fonction de leur distance par rapport à la fin de leur traversé l'intersection.
// le vehicule en tete de priorité doit passer l'intersection le plus vite possible.
// les vehicules suivant doivent ralentir si un vehicule prioritaire va se placer sur leurs trajectoire. (si ils sont sur la meme voie, considérer aussi le changement de voie)
// dès lors qu'un vehicule non prioritaire n'a plus d'obstacle il se doit de reprendre sa vitesse maximum (5 pixels par frame).
pub fn cars_insert_by_priority(car_arr: Vec<Car>, new_car: Car) -> Vec<Car> {
    let mut car_priority = Vec::<Car>::new();
    let distance_to_end_new_car = new_car.distance_to_end_intersection(0);
    /*if car_arr.len() == 0 {
        car_priority.push(new_car);
        return car_priority
    }*/
    let mut have_add = false;
    let mut start_len = car_arr.len();
    for a_car in car_arr {
        let distance_to_end_a_car = a_car.distance_to_end_intersection(0);
        if distance_to_end_new_car < distance_to_end_a_car && !have_add {
            car_priority.push(new_car);
            have_add = true;
        }
        car_priority.push(a_car);
    }
    if car_priority.len() == start_len {
        car_priority.push(new_car);
    }
    return car_priority
}

// retourn le Vec<Car> trié par priorité, un booléen indiquant si la voiture à passé l'intersection et la voiture qui a passé l'intersection
pub fn cars_distribute_by_priority(car_arr: Vec<Car>, car_passed_arr: Vec<Car>) -> (Vec<Car>, Vec<Car>) {  //-> (Vec<Car>, bool, Car) {
    let mut car_priority = Vec::<Car>::new();
    let mut out_car_passed_arr = car_passed_arr;
    //let mut car_passed_intersection_bool: bool = false;
    //let mut car_passed_intersection: Car = Car::new(Direction::Up, Voie::Droite, 0);

    let mut car_need_slow = Vec::<i32>::new();
    //let mut all_dist_col = Vec::<(i32, (i32, i32))>::new();//(car_number, (other_car_number, separing_distance))
    //let mut previous_car: Car = Car::new(Direction::Up, Voie::Droite, 0, 0);
    let mut i: i32 = 0;
    for &(mut a_car) in &car_arr {
        i += 1;
        //let mut car_dist_col = Vec::new();
        //if i > 1 {
            // collision_with contient une liste de tuple (car_collisioned, collision_zone)
            let mut okay: bool = true;
            for a_previous_car in &car_arr {
                if a_previous_car.direction == a_car.direction && a_previous_car.voie == a_car.voie {
                    //if a_previous_car.speed < a_car.speed {
                        let mut separing_distance: i32 = 0;
                        //if a_car.direction == Direction::Up || a_car.direction == Direction::Left {
                            separing_distance = a_car.distance_to_end_intersection(0) - a_previous_car.distance_to_end_intersection(0);
                        //} else if a_car.direction == Direction::Down || a_car.direction == Direction::Right {
                            //separing_distance = a_previous_car.distance_to_end_intersection(0) - a_car.distance_to_end_intersection(0);
                        //}
                        let mut secu_dist = 0;
                        if a_car.turned || a_previous_car.turned {
                            secu_dist = 2*car_length;
                        } else {
                            secu_dist = 6*car_length;
                        }
                        if separing_distance < secu_dist && separing_distance > 0 {
                            car_need_slow.push(a_car.number);
                            //car_dist_col.push((a_previous_car.number, separing_distance));
                            //a_car.speed = a_previous_car.speed;
                            okay = false;
                            break;
                        }
                    //}
                }
            }
            if okay {
                //println!("a");
                let mut collision_with = Vec::new();
                let mut ii: i32 = 0;
                for mut a_previous_car in &car_arr {
                    ii += 1;
                    // CHANGER CA,
                    // BESOIN DE TROUVER LE VEHICULE LE PLUS PROCHE DE LA COLLISION parmis tous les vehucules.
                    // lister les vehicules qui font collision,
                    // en fonction de la direction et de de la voie du self,
                    // definir chaque emplacement etant la priorité en fonction des dites direction et voie,
                    // where_collision_zone doit retourner le vehicules qui fait collision, puis on doit traiter ce qui est commenté si dessus après cette boucle for qui liste les vehicules qui font collision.
                    // simuler à la main des verification sur les distances du point d'impact, ca a l'air de merder surtout sur les voies du centre.
                    let collision_zone = a_car.where_collision_zone(*a_previous_car);
                    if collision_zone.len() != 0 && a_car.number != a_previous_car.number {
                        collision_with.push((a_previous_car, collision_zone, ii));
                    }
                }
                let mut priority_collision_zone = Vec::<i32>::new();
                match a_car.direction {
                    Direction::Up => {
                        match a_car.voie {
                            Voie::Centre => {
                                priority_collision_zone.push(16);
                                priority_collision_zone.push(8);
                                priority_collision_zone.push(4);
                            },
                            Voie::Gauche => {
                                priority_collision_zone.push(15);
                                priority_collision_zone.push(11);
                                priority_collision_zone.push(7);
                                priority_collision_zone.push(6);
                                priority_collision_zone.push(5);
                            },
                            _ => {}
                        }
                    },
                    Direction::Down => {
                        match a_car.voie {
                            Voie::Centre => {
                                priority_collision_zone.push(1);
                                priority_collision_zone.push(9);
                                priority_collision_zone.push(13);
                            },
                            Voie::Gauche => {
                                priority_collision_zone.push(2);
                                priority_collision_zone.push(6);
                                priority_collision_zone.push(10);
                                priority_collision_zone.push(11);
                                priority_collision_zone.push(12);
                            },
                            _ => {}
                        }
                    },
                    Direction::Left => {
                        match a_car.voie {
                            Voie::Centre => {
                                priority_collision_zone.push(4);
                                priority_collision_zone.push(2);
                                priority_collision_zone.push(1);
                            },
                            Voie::Gauche => {
                                priority_collision_zone.push(8);
                                priority_collision_zone.push(7);
                                priority_collision_zone.push(6);
                                priority_collision_zone.push(10);
                                priority_collision_zone.push(14);
                            },
                            _ => {}
                        }
                    },        //verticale
                    Direction::Right => {
                        match a_car.voie {
                            Voie::Centre => {
                                priority_collision_zone.push(13);
                                priority_collision_zone.push(15);
                                priority_collision_zone.push(16);
                            },
                            Voie::Gauche => {
                                priority_collision_zone.push(9);
                                priority_collision_zone.push(10);
                                priority_collision_zone.push(11);
                                priority_collision_zone.push(7);
                                priority_collision_zone.push(3);
                            },
                            _ => {}
                        }
                    },
                }

                //TCHECKER QUI RALENTIT PAR RAPPORT A QUI ET GENERER L4ECART DE VITESSE POUR ACCELERER ET PASS2 DEVANT SI BESOIN OU NE PAS RALENTIR SI L4AUTRE RALENTIT DEJA PAR RAPPORT A CELLE CI
                //let mut priority_collision = Car::new(Direction::Up, Voie::Droite, 0, 0);
                //let mut need_break = false;

                for car_collision in &collision_with {
                    for col_point in &car_collision.1 {

                        /*let car_d = a_car.distance_to_end_intersection(*col_point);
                        let other_d = car_collision.0.distance_to_end_intersection(*col_point);
                        let sep_d = car_d - other_d;
                        if sep_d >= 0 {
                            car_dist_col.push((car_collision.0.number, sep_d));
                        }*/

                        for prio_col_point in &priority_collision_zone {
                            if col_point == prio_col_point  && *col_point != 0 {
                                //let ratio_car: f64 = a_car.speed as f64 / car_collision.0.speed as f64;
                                //let ratio_other_car: f64 = car_collision.0.speed as f64 / a_car.speed as f64;
                                //let mut car_dist: i32 = (a_car.distance_to_end_intersection(*col_point) as f64 * ratio_other_car) as i32; //* (car_collision.0.speed/a_car.speed);
                                //let mut prio_dist: i32 = (car_collision.0.distance_to_end_intersection(*col_point) as f64 * ratio_car) as i32;//* (a_car.speed/car_collision.0.speed);
                                let mut prio_col_point: i32 = *col_point;
                                if car_collision.0.direction == Direction::Up || car_collision.0.direction == Direction::Down {
                                    if *col_point == 6 {
                                        prio_col_point = 11;
                                    } else if *col_point == 11 {
                                        prio_col_point = 6;
                                    }
                                } else if car_collision.0.direction == Direction::Right || car_collision.0.direction == Direction::Left {
                                    if *col_point == 7 {
                                        prio_col_point = 10
                                    } else if *col_point == 10 {
                                        prio_col_point = 7
                                    } 
                                }
                                let car_dist: i32 = a_car.distance_to_end_intersection(*col_point);
                                let mut prio_dist: i32 = car_collision.0.distance_to_end_intersection(prio_col_point);
                                let mut separing_distance: i32 = car_dist - prio_dist;
                                let mut secu_dist = (2*car_length) + 12;
                                println!("  1: car dist: {}, other dist: {}, car col point: {}, other col point {}", car_dist, prio_dist, *col_point, prio_col_point);
                                //if ((a_car.direction == Direction::Up || a_car.direction == Direction::Down) && (car_collision.0.direction == Direction::Up || car_collision.0.direction == Direction::Down))
                                //|| ((a_car.direction == Direction::Right || a_car.direction == Direction::Left) && (car_collision.0.direction == Direction::Right || car_collision.0.direction == Direction::Left)) {
                                    if separing_distance < secu_dist && separing_distance > 0 {
                                        println!("2: car number: {}, collision zone: {}, separing distance: {}", a_car.number, col_point, separing_distance);
                                        /*car_need_slow.push(a_car.number);
                                        if car_dist < 2*car_length {
                                            a_car.color = Color::BLACK;
                                        }
                                        //if car_dist < car_length + pointille_width {
                                            //a_car.color = Color::RED;
                                            //let mut b: Car = *car_collision.0;
                                            //b.color = Color::RED;
                                            //car_collision.0.color = Color::RED;
                                        //}
                                        //need_break = true;
                                        //break;*/
                                    } else if car_dist == prio_dist {//separing_distance == 0 {
                                        //mettre un partie du ci dessus la dedans
                                        /*println!("SAME DISTANCE QUI EST PRIORITAIRE");
                                        if i > car_collision.2 {
                                            car_need_slow.push(a_car.number);
                                        }*/
                                    }
                                //} else {
                                    /*if separing_distance < secu_dist && separing_distance > -secu_dist {
                                        println!("2: car number: {}, collision zone: {}, separing distance: {}", a_car.number, col_point, separing_distance);
                                        car_need_slow.push(a_car.number);
                                        if car_dist < 2*car_length {
                                            a_car.color = Color::BLACK;
                                        }
                                        //if car_dist < car_length + pointille_width {
                                            //a_car.color = Color::RED;
                                            //let mut b: Car = *car_collision.0;
                                            //b.color = Color::RED;
                                            //car_collision.0.color = Color::RED;
                                        //}
                                        //need_break = true;
                                        //break;
                                    } else if car_dist == prio_dist {//separing_distance == 0 {
                                        //mettre un partie du ci dessus la dedans
                                        println!("SAME DISTANCE QUI EST PRIORITAIRE");
                                        if i > car_collision.2 {
                                            car_need_slow.push(a_car.number);
                                        }
                                    }*/
                                //}
                                /*if separing_distance < secu_dist && separing_distance > 0 {
                                    println!("2: car number: {}, collision zone: {}, separing distance: {}", a_car.number, col_point, separing_distance);
                                    car_need_slow.push(a_car.number);
                                    if car_dist < 2*car_length {
                                        a_car.color = Color::BLACK;
                                    }
                                    //if car_dist < car_length + pointille_width {
                                        //a_car.color = Color::RED;
                                        //let mut b: Car = *car_collision.0;
                                        //b.color = Color::RED;
                                        //car_collision.0.color = Color::RED;
                                    //}
                                    //need_break = true;
                                    //break;
                                } else if car_dist == prio_dist {//separing_distance == 0 {
                                    //mettre un partie du ci dessus la dedans
                                    println!("SAME DISTANCE QUI EST PRIORITAIRE");
                                    if i > car_collision.2 {
                                        car_need_slow.push(a_car.number);
                                    }
                                }*/
                            }
                        }
                        /*if car_need_slow.len() != 0 {
                            println!("");
                        }*/
                        /*if need_break {
                            break;
                        }*/
                    }
                    /*if need_break {
                        break;
                    }*/
                    /*for a_collision in &priority_collision_zone {
                        if car_collision.1 == *a_collision {
                            priority_collision = car_collision.0;
                            break;
                        }
                    }*/
                    /*if priority_collision.number != 0 {
                        let mut car_dist = a_car.distance_to_end_intersection(car_collision.1);
                        let mut prio_dist = priority_collision.distance_to_end_intersection(car_collision.1);
                        // si l'autre voiture est prioritaire, si sa vitesse est inferieur à la mienne je me met à sa vitesse, 
                        //if prio_dist < car_dist {
                            //if priority_collision.speed < a_car.speed {
                                //a_car.speed = priority_collision.speed;
                            //}
                        //}
                        //println!("car_d: {}, prio_d: {}", car_dist, prio_dist);
                        /*if car_dist < 0 {
                            car_dist *= -1;
                        }
                        if prio_dist < 0 {
                            prio_dist *= -1;
                        }*/
                        //prio_dist est le point zero, separing distance est la distance entre a_car et le point zero
                        let mut separing_distance: i32 = car_dist - prio_dist;
                        /*let mut separing_distance: i32 = 0;
                        if car_dist > prio_dist {
                        //if i > car_collision.2 {
                            separing_distance = prio_dist - car_dist;
                        } else {
                            separing_distance = car_dist - prio_dist;
                        }*/
                        let mut secu_dist = (2*car_length) + 12;
                        /*if a_car.voie == Voie::Centre {
                            secu_dist = car_length + 12;
                        } else {
                            secu_dist = 2*car_length + 12;
                        }*/
                        if separing_distance < secu_dist && separing_distance > 0 {//&& car_dist >= 0 {//&& prio_dist >= 0 {//&& separing_distance > -(car_length + 25) {
                            println!("car number: {}, collision zone: {}, separing distance: {}", a_car.number, car_collision.1, separing_distance);
                           // if car_dist > prio_dist {
                                car_need_slow.push(a_car.number);
                                if car_dist < car_length + pointille_width {
                                    a_car.color = Color::RED;
                                }
                                break;
                            /*} else if prio_dist > car_dist {
                                car_need_slow.push(priority_collision.number);
                                /*println!("priority_collision ralentit");
                                //collision.0 ralentit
                                if priority_collision.speed > 1 {

                                    priority_collision.speed -= 2;
                                }*/
                                break;*/
                            /*} else {
                                if i > car_collision.2 {
                                    car_need_slow.push(a_car.number);
                                    /*if a_car.speed > 1 {
                                        a_car.speed -= 2;
                                    }*/
                                }/* else {
                                    car_need_slow.push(priority_collision.number);
                                    /*if priority_collision.speed > 1 {
                                        priority_collision.speed -= 2;
                                    }*/
                                }*/
                                break;
                            }*/
                        } else if separing_distance == 0 {
                            //mettre un partie du ci dessus la dedans
                            println!("SAME DISTANCE QUI EST PRIORITAIRE");
                            if i > car_collision.2 {
                                car_need_slow.push(a_car.number);
                            }
                        }/* else {
                            a_car.max_speed();
                        }*/
                        //tcheck hitbox
                        /*if car_dist < 52 && car_dist > 0 {
                            if separing_distance < secu_dist && separing_distance > 0 {
                                a_car.color = Color::RED;
                            }
                        }*/
                    }*/
                }
                /*if priority_collision.number == 0 {
                    a_car.max_speed();
                }*/
            }
        /*} else {
            a_car.max_speed();
        }*/
        if a_car.distance_to_end_intersection(0) < 0 {
            //a_car.max_speed();
            //car_passed_intersection_bool = true;
            //car_passed_intersection = a_car;
            out_car_passed_arr.push(a_car);
        } else {
            car_priority.push(a_car);
            /*let mut prioryti_copy = Vec::<Car>::new();
            let mut have_append = false;
            for ordened_car in &car_priority {
                if a_car.distance_to_end_intersection(0) < ordened_car.distance_to_end_intersection(0)  && !have_append{
                    prioryti_copy.push(a_car);
                    have_append = true;
                }
                prioryti_copy.push(*ordened_car);
            }
            if !have_append {
                prioryti_copy.push(a_car);
            }
            car_priority = prioryti_copy;*/
        }
        //previous_car = a_car;
    }

    //selectionner la first car de chaque voie
    //lister les collisions
    //calculer la distance de collisions
    //let mut min: i32 = 0;
    let mut first_car: i32 = 0;
    for a_car in &car_priority {
        let dist = a_car.distance_to_end_intersection(0);
        let mut first = true;
        for other_car in &car_priority {
            let other_dist = other_car.distance_to_end_intersection(0);
            if other_car.number != a_car.number {
                if other_dist < dist {
                    first = false;
                    break
                }
            }
        }
        if first {
            first_car = a_car.number;
            /*for other_car in &car_priority {m
                //let a = *a_car;
                let col_point = a_car.where_collision_zone(*other_car);
                for col_zone in &col_point {
                    if *col_zone != 0 {
                        let car_dist = a_car.distance_to_end_intersection(*col_zone);
                        let other_dist = other_car.distance_to_end_intersection(*col_zone);
                        let sep_dist = other_dist - car_dist;
                        if sep_dist < 2*car_length + 12 && sep_dist > 0 {
                            car_need_slow.push(other_car.number);
                        }
                        // si other_car atteint le point de collision avant a_car et est prioritaire,
                        // si une autre voiture est prioritaire sur other_car et ainsi de suite pour trouver la vrai priorité,
                        // cela devrait finir sur une recursion qui revient parfois sur a_car qui en ce cas est prioritaire.
                        if sep_dist <= 0 {
                            // other_car devient prioritaire si elle n'a pas de collision sinon rechercher la voiture prioritaire sur other_car,
                            // si l'on retourne sur l'une des voitures precédente (a_car, other_car, third_car, etc...) la priorité revient à cette voiture.
                            for third_car in & car_priority {
                                let third_col_p = a_car.where_collision_zone(*third_car);
                                for third_col in &third_col_p {
                                    if *third_col != 0 {
                                        let third_dist = third_car.distance_to_end_intersection(*third_col);
                                        let other_dist = other_car.distance_to_end_intersection(*third_col);
                                        let third_sep_dist = other_dist - third_dist;
                                        if third_sep_dist < 2*car_length + 12 && third_sep_dist > 0 {
                                            car_need_slow.push(other_car.number);                                            
                                        } else if sep_dist <= 0 {
                                            // third_car devient prioritaire si elle n'a pas de collision sinon rechercher la voiture prioritaire sur third_car,
                                            // si l'on retourne sur l'une des voitures precédente (a_car, other_car, etc..) la priorité revient à cette voiture.
                                            etc...
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }*/
            //a_car.max_speed();
            //a_car.color = Color::GREEN;
            //break;
        }
    }

    for mut a_car in &mut car_priority {
        if a_car.number == first_car {
            a_car.max_speed();
            a_car.color = Color::GREEN;
        } else {
            a_car.min_speed();
            /*let mut have_slow = false;
            for car_number in &car_need_slow {
                if a_car.number == *car_number {
                    a_car.min_speed();
                    a_car.color = Color::RED;
                    /*if a_car.speed > 1 {
                        a_car.speed -= 2;
                    }*/
                    have_slow = true;
                }
            }
            if !have_slow {
                a_car.max_speed();
                a_car.color = Color::BLUE;
            }*/
        }
    }
    //println!("{} car number", i);
    return (car_priority, out_car_passed_arr)
    //return (car_priority, car_passed_intersection_bool, car_passed_intersection)
}

pub fn arr_contains(arr: Vec::<i32>, number: i32) -> bool {
    for a_number in &arr {
        if *a_number == number {
            return true;
        }
    }
    return false;
}

/*pub fn do_slow_other_car(car_arr: &Vec::<Car>, car_number: i32) {
    for &(mut a_car) in car_arr {
        if a_car.number != car_number {
            if a_car.speed > 1 {
                a_car.speed -= 2;
            }
        }
    }
}*/