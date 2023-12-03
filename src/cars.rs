
use sdl2::pixels::Color;
use crate::road::Direction;
use crate::road::Voie;


pub static pointille_width: i32 = 2;
pub static pointille_length: i32 = 20;

pub static voie_width: i32 = 50;
pub static voie_length: i32 = 1000;

pub static road_width: i32 = 3*voie_width + 4*pointille_width;
pub static road_length: i32 = 1000;

pub static carrefour_start: i32 = (1000-road_width)/2;
pub static carrefour_end: i32 = carrefour_start + road_width;

pub static car_width: i32 = 50;
pub static car_length: i32 = 50;


static mut base_speed: i32 = 5;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coordonnees {
    pub x: i32,
    pub y: i32,
}
impl Coordonnees {
    pub fn new(x: i32, y: i32) -> Coordonnees {
        return Coordonnees { x: x, y: y }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rectangle {
    pub position: Coordonnees,
    pub width: i32,
    pub height: i32,
}
impl Rectangle {
    pub fn new(position_x: i32, position_y: i32, width: i32, height: i32) -> Rectangle {
        return Rectangle { position: Coordonnees::new(position_x, position_y), width: width, height: height }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Car {
    pub number: i32,
    pub direction: Direction,
    pub voie: Voie,
    pub hitbox: Rectangle,
    pub color: Color,
    pub speed: i32,
    pub distance: i32,
    pub turned: bool,
}
impl Car {
    pub fn new(dir: Direction, voi: Voie, last_car_position: i32, car_number: i32) -> Self {
        
        let mut pos = Self::car_spawn_position(dir, voi);
        match dir {
            Direction::Up => {
                if last_car_position > 1000 - 100 {
                    pos.y = last_car_position + 6*car_length;
                }
            }
            Direction::Down => {
                if last_car_position < 0 + 100 {
                    pos.y = last_car_position - 6*car_length;
                }
            }
            Direction::Left => {
                if last_car_position > 1000 - 100 {
                    pos.x = last_car_position + 6*car_length;
                }
            }
            Direction::Right => {
                if last_car_position  < 0 + 100 {
                    pos.x = last_car_position - 6*car_length;
                }
            }
        }
        Self {
            number: car_number,
            direction: dir,
            voie: voi,
            hitbox: Rectangle::new(pos.x, pos.y, car_width, car_length),
            color: Color::RGB(0, 0, 120),
            speed: unsafe { base_speed },
            distance: 0,
            turned: false,
        }
    }
    
    pub fn car_spawn_position(dir: Direction, voi: Voie) -> Coordonnees {
        match dir {
            Direction::Up => match  voi {
                Voie::Droite => Coordonnees::new(500+(2*voie_width + 3*pointille_width), 1000),
                Voie::Centre => Coordonnees::new(500+(voie_width + 2*pointille_width), 1000),
                Voie::Gauche => Coordonnees::new(500+(pointille_width), 1000),
            },
            Direction::Down => match voi {
                Voie::Droite => Coordonnees::new(500-(3*voie_width + 3*pointille_width), 0-car_length),
                Voie::Centre => Coordonnees::new(500-(2*voie_width + 2*pointille_width), 0-car_length),
                Voie::Gauche => Coordonnees::new(500-(voie_width + pointille_width), 0-car_length),
            },
            Direction::Left => match voi {
                Voie::Droite => Coordonnees::new(1000, 500-(3*voie_width + 3*pointille_width)),
                Voie::Centre => Coordonnees::new(1000, 500-(2*voie_width + 2*pointille_width)),
                Voie::Gauche => Coordonnees::new(1000, 500-(voie_width + pointille_width)),
            },
            Direction::Right => match voi {
                Voie::Droite => Coordonnees::new(0-car_width, 500+(2*voie_width + 3*pointille_width)),
                Voie::Centre => Coordonnees::new(0-car_width, 500+(voie_width + 2*pointille_width)),
                Voie::Gauche => Coordonnees::new(0-car_width, 500+(pointille_width)),
            },
        }
    }
    
    pub fn outside_screen(&mut self) -> bool {
        match self.direction {
            Direction::Up => return self.hitbox.position.y < 0-car_length,
            Direction::Down => return self.hitbox.position.y > 1000+car_length,
            Direction::Left => return self.hitbox.position.x < 0-car_length,
            Direction::Right => return self.hitbox.position.x > 1000+car_length,
        }
    }

    pub fn min_speed(&mut self) {
        self.speed = 1;
    }

    pub fn max_speed(&mut self) {
        self.speed = 10;
    }
    
    pub fn direction_speed(&mut self) -> Coordonnees {
        match self.direction {
            Direction::Up => Coordonnees::new(0, -self.speed),
            Direction::Down => Coordonnees::new(0, self.speed),
            Direction::Left => Coordonnees::new(-self.speed, 0),
            Direction::Right => Coordonnees::new(self.speed, 0),
        }
    }

    pub fn follow_previous(&mut self, previous_car: &Car) {
        self.speed = previous_car.speed;
    }

    pub fn in_screen(self) -> bool {
        if self.hitbox.position.x < 0 || self.hitbox.position.x > 1000 || self.hitbox.position.y < 0 || self.hitbox.position.y > 1000 {
            return false;
        }
        return true;
    }

    // focused_zone represente le point de croisesment de deux direction sur un dite voie
    pub fn distance_to_end_intersection(self, focused_zone: i32) -> i32 {
        // collision_zone_n = tuple (start, end)
        //calculer la distance par rapport à la fin de l'intersection
        let mut d_x: i32 = 0;
        let mut d_y: i32 = 0;
        if focused_zone == 0 {
            match self.direction {
                Direction::Up => {
                    match self.voie {
                        Voie::Centre => {
                            d_x = 0;
                            d_y = self.hitbox.position.y - (500 - 2*voie_width - 3*pointille_width - car_length);
                        },
                        Voie::Gauche => {
                            if !self.turned {
                                d_x = self.hitbox.position.x - (500 - 2*voie_width - 3*pointille_width - car_length);
                                d_y = self.hitbox.position.y - (500 - voie_width - pointille_width);
                            } else {
                                d_x = (500 + pointille_width) - self.hitbox.position.x;
                                d_y = self.hitbox.position.y - (500 - 2*voie_width - 3*pointille_width - car_length);
                            }
                        },
                        _ => {},
                    }
                },
                Direction::Down => {
                    match self.voie {
                        Voie::Centre => {
                            d_x = 0;
                            d_y = (500 + 2*voie_width + 3*pointille_width) - self.hitbox.position.y;
                        },
                        Voie::Gauche => {
                            if !self.turned {
                                d_x = (500 + 2*voie_width + 3*pointille_width) - self.hitbox.position.x;
                                d_y = (500 + pointille_width) - self.hitbox.position.y;
                            } else {
                                d_x = self.hitbox.position.x - (500 - voie_width - pointille_width);
                                d_y = (500 + 2*voie_width + 3*pointille_width) - self.hitbox.position.y;
                            }
                        },
                        _ => {},
                    }
                },
                Direction::Left => {
                    match self.voie {
                        Voie::Centre => {
                            d_x = self.hitbox.position.x - (500 - 2*voie_width - 3*pointille_width - car_length);
                            d_y = 0;
                        },
                        Voie::Gauche => {
                            if !self.turned {
                                d_x = self.hitbox.position.x - (500 - voie_width - pointille_width);
                                d_y = (500 + 2*voie_width + 3*pointille_width) - self.hitbox.position.y;
                            } else {
                                d_x = self.hitbox.position.x - (500 - 2*voie_width - 3*pointille_width - car_length);
                                d_y = self.hitbox.position.y - (500 - voie_width - pointille_width);
                            }
                        },
                        _ => {},
                    }
                },
                Direction::Right => {
                    match self.voie {
                        Voie::Centre => {
                            d_x = (500 + 2*voie_width + 3*pointille_width) - self.hitbox.position.x;
                            d_y = 0;
                        },
                        Voie::Gauche => {
                            if !self.turned {
                                d_x = (500 + pointille_width) - self.hitbox.position.x;
                                d_y = self.hitbox.position.y - (500 - 2*voie_width - 3*pointille_width - car_length);
                            } else {
                                d_x = (500 + 2*voie_width + 3*pointille_width) - self.hitbox.position.x; //self.hitbox.position.x - (500 - voie_width - pointille_width);
                                d_y = (500 + pointille_width) - self.hitbox.position.y; //(500 + 2*voie_width + 3*pointille_width) - self.hitbox.position.y;
                            }
                        },
                        _ => {},
                    }
                },
            }
        } else {
            let mut end = Coordonnees::new(0, 0);//(Coordonnees::new(0, 0), Coordonnees::new(0, 0));
            // parameter x
            if focused_zone == 1 || focused_zone == 5 || focused_zone == 9 || focused_zone == 13 {
                end.x = 500 - 2*voie_width - 2*pointille_width;
                
            } else if focused_zone == 2 || focused_zone == 6 || focused_zone == 10 || focused_zone == 14 {
                end.x = 500 - voie_width - pointille_width;
               
            } else if focused_zone == 3 || focused_zone == 7 || focused_zone == 11 || focused_zone == 15  {
                end.x = 500 + pointille_width;
               
            } else if focused_zone == 4 || focused_zone == 8 || focused_zone == 12 || focused_zone == 16 {
                end.x = 500 + voie_width + 2*pointille_width;
                
            } 
            // parameter y
            if focused_zone == 1 || focused_zone == 2 || focused_zone == 3 || focused_zone == 4 {
                end.y = 500 - 2*voie_width - 2*pointille_width;
               
            } else if focused_zone == 5 || focused_zone == 6 || focused_zone == 7 || focused_zone == 8 {
                end.y = 500 - voie_width - pointille_width;
                
            } else if focused_zone == 9 || focused_zone == 10 || focused_zone == 11 || focused_zone == 12 {
                end.y = 500 + pointille_width;
               
            } else if focused_zone == 13 || focused_zone == 14 || focused_zone == 15 || focused_zone == 16 {
                end.y = 500 + voie_width + 2*pointille_width;
                
            }
            if self.direction == Direction::Up {
                d_x = self.hitbox.position.x - end.x;
                d_y = self.hitbox.position.y - end.y;
            } else if self.direction == Direction::Down {
                d_x = end.x - self.hitbox.position.x;
                d_y = end.y - self.hitbox.position.y;
            } else if self.direction == Direction::Left {
                d_x = self.hitbox.position.x - end.x;
                d_y = end.y - self.hitbox.position.y;
            } else if self.direction == Direction::Right {
                d_x = end.x - self.hitbox.position.x;
                d_y = self.hitbox.position.y - end.y;
            }
        }
        
        return d_x + d_y;
    }

    // retourne la zone de collision, de 1 à 16
    pub fn where_collision_zone(self, a_previous_car: Car) -> Vec::<i32> {
        let mut collision_zone: Vec::<i32> = Vec::new();
        
        if self.voie == Voie::Droite || a_previous_car.voie == Voie::Droite {
            collision_zone.push(0);
            return collision_zone;
        }

        
        
        match self.direction {
            Direction::Up => {
                match self.voie {
                    Voie::Centre => {
                        //
                        match a_previous_car.direction {
                            Direction::Right => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.x < 500 + 2*voie_width + 3*pointille_width {
                                            //collision_zone = 16;
                                            collision_zone.push(16);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            Direction::Left => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.x > 500 + pointille_width {
                                            //collision_zone = 4;
                                            collision_zone.push(4);
                                        }
                                    },
                                    Voie::Gauche => {
                                        if a_previous_car.hitbox.position.x > 500 + pointille_width {
                                            //collision_zone = 8;
                                            collision_zone.push(8);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            _ => {collision_zone.push(0);},
                        }
                        //
                    },
                    Voie::Gauche => {
                        //
                        match a_previous_car.direction {
                            Direction::Right => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.x < 500 + voie_width + 2*pointille_width {
                                            //collision_zone = 15;
                                            collision_zone.push(15);
                                        }
                                    },
                                    Voie::Gauche => {
                                        if a_previous_car.hitbox.position.x < 500 + voie_width + 2*pointille_width {
                                            //collision_zone = 11;
                                            collision_zone.push(11);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            Direction::Down => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.y < 500 + voie_width + pointille_width {
                                            //collision_zone = 5;
                                            collision_zone.push(5);
                                        }
                                    },
                                    Voie::Gauche => {
                                        
                                            if a_previous_car.hitbox.position.y < 500 + voie_width + pointille_width {
                                                //collision_zone = 6;//11
                                                if !self.turned && !a_previous_car.turned {
                                                    collision_zone.push(6);
                                                    //collision_zone.push(11);
                                                }
                                            }
                                        
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            Direction::Left => {
                                match a_previous_car.voie {
                                    Voie::Gauche => {
                                        if a_previous_car.hitbox.position.x > 500 + voie_width + 2*pointille_width {
                                            //collision_zone = 7;
                                            collision_zone.push(7);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            _ => {collision_zone.push(0);},
                        }
                        //
                    },
                    _ => {collision_zone.push(0);},
                }
            },
            Direction::Down => {
                match self.voie {
                    Voie::Centre => {
                        //
                        match a_previous_car.direction {
                            Direction::Right => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.x < 500 - voie_width - pointille_width {
                                            //collision_zone = 13;
                                            collision_zone.push(13);
                                        }
                                    },
                                    Voie::Gauche => {
                                        if a_previous_car.hitbox.position.x < 500 - voie_width - pointille_width {
                                            //collision_zone = 9;
                                            collision_zone.push(9);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            Direction::Left => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.x > 500 - 3*voie_width - 3*pointille_width {
                                            //collision_zone = 1;
                                            collision_zone.push(1);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            _ => {collision_zone.push(0);},
                        }
                        //
                    },
                    Voie::Gauche => {
                        //
                        match a_previous_car.direction {
                            Direction::Up => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.y > 500 - voie_width - pointille_width {
                                            //collision_zone = 12;
                                            collision_zone.push(12);
                                        }
                                    },
                                    Voie::Gauche => {
                                        
                                            if a_previous_car.hitbox.position.y > 500 - voie_width - pointille_width {
                                                //collision_zone = 11;//6
                                                if !self.turned && !a_previous_car.turned {
                                                    collision_zone.push(11);
                                                    //collision_zone.push(6);
                                                }
                                            }
                                        
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            Direction::Right => {
                                match a_previous_car.voie {
                                    Voie::Gauche => {
                                        if a_previous_car.hitbox.position.x < 500 + pointille_width {
                                            //collision_zone = 10;
                                            collision_zone.push(10);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            Direction::Left => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.x > 500 - 2*voie_width - 2*pointille_width {
                                            //collision_zone = 2;
                                            collision_zone.push(2);
                                        }
                                    },
                                    Voie::Gauche => {
                                        if a_previous_car.hitbox.position.x > 500 - 2*voie_width - 2*pointille_width {
                                            //collision_zone = 6;
                                            collision_zone.push(6);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            _ => {collision_zone.push(0);},
                        }
                        //
                    },
                    _ => {collision_zone.push(0);}
                }
            },
            Direction::Right => {
                match self.voie {
                    Voie::Centre => {
                        //
                        match a_previous_car.direction {
                            Direction::Up => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.y > 500 + pointille_width {
                                            //collision_zone = 16;
                                            collision_zone.push(16);
                                        }
                                    },
                                    Voie::Gauche => {
                                        if a_previous_car.hitbox.position.y > 500 + pointille_width {
                                            //collision_zone = 15;
                                            collision_zone.push(15);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            Direction::Down => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.y < 500 +2*voie_width + 3*pointille_width {
                                            //collision_zone = 13;
                                            collision_zone.push(13);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            _ => {collision_zone.push(0);},
                        }
                        //
                    },
                    Voie::Gauche => {
                        //
                        match a_previous_car.direction {
                            Direction::Up => {
                                match a_previous_car.voie {
                                    Voie::Gauche => {
                                        if a_previous_car.hitbox.position.y > 500 + voie_width + pointille_width {
                                            //collision_zone = 11;
                                            collision_zone.push(11);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            Direction::Down => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.y < 500 + voie_width + 2*pointille_width {
                                            //collision_zone = 9;
                                            collision_zone.push(9);
                                        }
                                    },
                                    Voie::Gauche => {
                                        if a_previous_car.hitbox.position.y < 500 + voie_width + 2*pointille_width {
                                            //collision_zone = 10;
                                            collision_zone.push(10);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            Direction::Left => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.x > 500 - voie_width - pointille_width {
                                            //collision_zone = 3;
                                            collision_zone.push(3);
                                        }
                                    },
                                    Voie::Gauche => {
                                        //if !a_previous_car.turned {
                                            if a_previous_car.hitbox.position.x > 500 - voie_width - pointille_width {
                                                //collision_zone = 7;//10
                                                if !self.turned && !a_previous_car.turned {
                                                    collision_zone.push(7);
                                                    //collision_zone.push(10);
                                                }
                                            }
                                        //}
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            _ => {collision_zone.push(0);},
                        }
                        //
                    },
                    _ => {},
                }
            },
            Direction::Left => {
                match self.voie {
                    Voie::Centre => {
                        //
                        match a_previous_car.direction {
                            Direction::Up => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.y > 500 + 3*voie_width + 3*pointille_width {
                                            //collision_zone = 4;
                                            collision_zone.push(4);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            Direction::Down => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.y < 500 - voie_width - pointille_width {
                                            //collision_zone = 1;
                                            collision_zone.push(1);
                                        }
                                    },
                                    Voie::Gauche => {
                                        if a_previous_car.hitbox.position.y < 500 - voie_width - pointille_width {
                                            //collision_zone = 2;
                                            collision_zone.push(2);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            _ => {collision_zone.push(0);},
                        }
                        //
                    },
                    Voie::Gauche => {
                        //
                        match a_previous_car.direction {
                            Direction::Up => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.y > 500 - 2*voie_width - 2*pointille_width {
                                            //collision_zone = 8;
                                            collision_zone.push(8);
                                        }
                                    },
                                    Voie::Gauche => {
                                        if a_previous_car.hitbox.position.y > 500 - 2*voie_width - 2*pointille_width {
                                            //collision_zone = 7;
                                            collision_zone.push(7);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            Direction::Down => {
                                match a_previous_car.voie {
                                    Voie::Gauche => {
                                        if a_previous_car.hitbox.position.y < 500 + pointille_width {
                                            //collision_zone = 6;
                                            collision_zone.push(6);
                                        }
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            Direction::Right => {
                                match a_previous_car.voie {
                                    Voie::Centre => {
                                        if a_previous_car.hitbox.position.x < 500 + pointille_width {
                                            //collision_zone = 14;
                                            collision_zone.push(14);
                                        }
                                    },
                                    Voie::Gauche => {
                                        //if !a_previous_car.turned {
                                            if a_previous_car.hitbox.position.x < 500 + pointille_width {
                                                //collision_zone = 10;//7
                                                if !self.turned && !a_previous_car.turned {
                                                    collision_zone.push(10);
                                                    //collision_zone.push(7);
                                                }
                                            }
                                        //}
                                    },
                                    _ => {collision_zone.push(0);},
                                }
                            },
                            _ => {collision_zone.push(0);},
                        }
                        //
                    },
                    _ => {collision_zone.push(0);},
                }
            },
        }
        

                return collision_zone
            
    }

    pub fn do_move(&mut self) {
        let mut speed = self.direction_speed();
        self.hitbox.position.x += speed.x;
        self.hitbox.position.y += speed.y;
        if speed.x <0 {
            speed.x *= -1;
        }
        if speed.y <0 {
            speed.y *= -1;
        }
        self.distance += speed.x + speed.y;
        if !self.turned {
            match self.direction {
                Direction::Up => match self.voie {
                    Voie::Droite => {
                        if self.hitbox.position.y < 500+(2*voie_width + 3*pointille_width) {
                            self.hitbox.position.y = 500+(2*voie_width + 3*pointille_width);
                            self.direction = Direction::Right;
                            self.voie = Voie::Droite;
                            self.turned = true;
                        }
                    }
                    Voie::Centre => {}
                    Voie::Gauche => {
                        if self.hitbox.position.y < 500-(voie_width+pointille_width) {
                            self.hitbox.position.y = 500-(voie_width+pointille_width);
                            self.direction = Direction::Left;
                            self.voie = Voie::Gauche;
                            self.turned = true;
                        }
                    }
                },
                Direction::Down => match self.voie {
                    Voie::Droite => {
                        if self.hitbox.position.y > 500-(3*voie_width + 3*pointille_width) {
                            self.hitbox.position.y = 500-(3*voie_width + 3*pointille_width);
                            self.direction = Direction::Left;
                            self.voie = Voie::Droite;
                            self.turned = true;
                        }
                    }
                    Voie::Centre => {},
                    Voie::Gauche => {
                        if self.hitbox.position.y > 500+(pointille_width) {
                            self.hitbox.position.y = 500+(pointille_width);
                            self.direction = Direction::Right;
                            self.voie = Voie::Gauche;
                            self.turned = true;
                        }
                    }
                },
                Direction::Right => match self.voie {
                    Voie::Droite => {
                        if self.hitbox.position.x > 500-(3*voie_width + 3*pointille_width) {
                            self.hitbox.position.x = 500-(3*voie_width + 3*pointille_width);
                            self.direction = Direction::Down;
                            self.voie = Voie::Droite;
                            self.turned = true;
                        }
                    }
                    Voie::Centre => {},
                    Voie::Gauche => {
                        if self.hitbox.position.x > 500+(pointille_width) {
                            self.hitbox.position.x = 500+(pointille_width);
                            self.direction = Direction::Up;
                            self.voie = Voie::Gauche;
                            self.turned = true;
                        }
                    }
                },
                Direction::Left => match self.voie {
                    Voie::Droite => {
                        if self.hitbox.position.x < 500+(2*voie_width + 3*pointille_width) {
                            self.hitbox.position.x = 500+(2*voie_width + 3*pointille_width);
                            self.direction = Direction::Up;
                            self.voie = Voie::Droite;
                            self.turned = true;
                        }
                    }
                    Voie::Centre => {},
                    Voie::Gauche => {
                        if self.hitbox.position.x < 500-(voie_width+pointille_width) {
                            self.hitbox.position.x = 500-(voie_width+pointille_width);
                            self.direction = Direction::Down;
                            self.voie = Voie::Gauche;
                            self.turned = true;
                        }
                    }
                },
            };
        }
    }
}