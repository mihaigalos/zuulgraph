extern crate rand;
use rand::Rng;

pub struct RandomColorGenerator {}
impl RandomColorGenerator {
    pub fn get_random_color() -> std::string::String {
        let red = format!("{:X}", rand::thread_rng().gen::<u8>());
        let green = format!("{:X}", rand::thread_rng().gen::<u8>());
        let blue = format!("{:X}", rand::thread_rng().gen::<u8>());
        let hex_color = red + &green + &blue;

        hex_color
    }
}
