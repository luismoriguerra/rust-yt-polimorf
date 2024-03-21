trait LandCapable {
    fn drive(&self) {
        println!("Driving on land");
    }
}

trait WaterCapable {
    fn float(&self) {
        println!("Floating on water");
    }
}

trait Amphibious: LandCapable + WaterCapable {}

struct Hovercraft;
impl Amphibious for Hovercraft {}
impl LandCapable for Hovercraft {}
impl WaterCapable for Hovercraft {}

struct Sedan;
impl LandCapable for Sedan {}

struct SUV;
impl LandCapable for SUV {}

// dyn or impl
// it depends on the use case
// dynamic dispatch
// dyn is used when you want to use trait objects
// runtime penalty

// static dispatch
// impl is used when you want to use generics
fn road_trip(car: &impl LandCapable) {
    car.drive();
}

fn traverse_Frozen_Lake(vehicle: &impl Amphibious) {
    vehicle.drive();
    vehicle.float();
}

fn main() {
    let car = Sedan;
    road_trip(&car);

    let hc = Hovercraft;
    traverse_Frozen_Lake(&hc);
}
