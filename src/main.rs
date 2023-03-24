enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Timing {
    fn duration(&self) -> u8;
}

impl Timing for TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Red => 50,
            TrafficLight::Green => 40,
            TrafficLight::Yellow => 8,
        }
    }
}
fn main() {
    let red_light = TrafficLight::Red;
    let green_light = TrafficLight::Green;
    let yellow_light = TrafficLight::Yellow;
    
    println!("Red light duration: {} seconds", red_light.duration());
    println!("Green light duration: {} seconds", green_light.duration());
    println!("Yellow light duration: {} seconds", yellow_light.duration());
}
