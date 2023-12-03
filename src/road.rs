extern crate rand; 

use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Voie {
    Droite,
    Centre,
    Gauche,
}
impl Voie {
    pub fn rand() -> Voie {
        let mut randhome = rand::thread_rng();
        match randhome.gen_range(0..3) {
            0 => Voie::Droite,
            1 => Voie::Centre,
            2 => Voie::Gauche,
            i32::MIN..=-1_i32 | 3_i32..=i32::MAX => todo!()
        }
    }
}
