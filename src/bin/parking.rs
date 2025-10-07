// Goals: Design a parking lot using object-oriented principles

// Here are a few methods that you should be able to run:

// Tell us how many spots are remaining
// Tell us how many total spots are in the parking lot
// Tell us when the parking lot is full
// Tell us when the parking lot is empty
// Tell us when certain spots are full e.g. when all motorcycle spots are taken
// Tell us how many spots vans are taking up
// Assumptions:

// The parking lot can hold motorcycles, cars and vans
// The parking lot has motorcycle spots, car spots and large spots
// A motorcycle can park in any spot
// A car can park in a single compact spot, or a regular spot
// A van can park, but it will take up 3 regular spots
// These are just a few assumptions. Feel free to ask your interviewer about more assumptions as needed

// add new

// Problem: Design and implement a ticketing system that issues a ticket when a vehicle enters and records entry time. When the vehicle exits, the system calculates parking fee based on duration and vehicle type.


// design parking lot using oops concepts

use std::collections::HashMap;

use chrono::Utc;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vehicle {
    Motorcycle,
    Car,
    Van,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpotType {
    Motorcycle,
    Compact,
    Large,
}

#[derive(Debug, Clone)]
struct Spot {
    spot_type: SpotType,
    occupied_by: Option<VehicleEntry>,
}
#[derive(Debug, Clone)]
pub struct VehicleEntry {
    vehicle: Vehicle,
    number_plate: String,
    entry_time: i64,   // utc::now().timestamp()   // an integer representing seconds since epoch timestamp
}

impl Spot {
    fn is_available(&self) -> bool {
        self.occupied_by.is_none()
    }
}

#[derive(Debug)]
struct ParkingLot {
    motorcycle_spots: Vec<Spot>,
    compact_spots: Vec<Spot>,
    large_spots: Vec<Spot>,
}

impl ParkingLot {
    fn new(motorcycle: usize, compact: usize, large: usize) -> Self {
        Self {
            motorcycle_spots: vec![
                Spot {
                    spot_type: SpotType::Motorcycle,
                    occupied_by: None
                };
                motorcycle
            ],
            compact_spots: vec![
                Spot {
                    spot_type: SpotType::Compact,
                    occupied_by: None
                };
                compact
            ],
            large_spots: vec![
                Spot {
                    spot_type: SpotType::Large,
                    occupied_by: None
                };
                large
            ],
        }
    }
    // total spots
    fn total_spots(&self) -> usize {
        self.motorcycle_spots.len() + self.compact_spots.len() + self.large_spots.len()
    }

    // remaining spots
    fn remaining_spots(&self) -> usize {
        self.motorcycle_spots
            .iter()
            .filter(|s| s.is_available())
            .count()
            + self
                .compact_spots
                .iter()
                .filter(|s| s.is_available())
                .count()
            + self.large_spots.iter().filter(|s| s.is_available()).count()
    }
    // Is parking lot full?
    fn is_full(&self) -> bool {
        self.remaining_spots() == 0
    }

    // is parking lot empty
    fn is_empty(&self) -> bool {
        self.motorcycle_spots.iter().all(|s| s.is_available())
            && self.compact_spots.iter().all(|s| s.is_available())
            && self.large_spots.iter().all(|s| s.is_available())
    }

    // are all the motorcycle spots full

    fn motorcycle_full(&self) -> bool {
        self.motorcycle_spots.iter().all(|s| !s.is_available())
    }

    // are all compact spaces full
    fn compact_full(&self) -> bool {
        self.compact_spots.iter().all(|s| !s.is_available())
    }
    // are all large spaces full
    fn large_full(&self) -> bool {
        self.large_spots.iter().all(|s| !s.is_available())
    }

    // how many spots are vans taking up
    fn van_spots_taken(&self) -> usize {
        self.large_spots
            .iter()
            .filter(|s| s.occupied_by.is_some() && s.occupied_by.as_ref().unwrap().vehicle == Vehicle::Van)
            .count()
    }

    // park vehicle
    fn park(&mut self, vehicle_type: Vehicle, number_plate: String) -> bool {
        match vehicle_type {
            Vehicle::Motorcycle => {
                if let Some(spot) = self.motorcycle_spots.iter_mut().find(|s| s.is_available()) {
                    spot.occupied_by = Some(VehicleEntry {
                        vehicle: Vehicle::Motorcycle,
                        number_plate,
                        entry_time: Utc::now().timestamp(),
                    });
                    return true;
                }
                //  else if let Some(spot) = self.compact_spots.iter_mut().find(|s| s.is_available())
                // {
                //     spot.occupied_by = Some(VehicleEntry {
                //         vehicle: Vehicle::Motorcycle,
                //         number_plate,
                //         entry_time: Utc::now().timestamp(),
                //     });
                //     return true;
                // } else if let Some(spot) = self.large_spots.iter_mut().find(|s| s.is_available()) {
                //     spot.occupied_by = Some(VehicleEntry {
                //         vehicle: Vehicle::Motorcycle,
                //         number_plate,
                //         entry_time: Utc::now().timestamp(),
                //     });
                //     return true;
                // }
            }
            Vehicle::Car => {
                if let Some(spot) = self.compact_spots.iter_mut().find(|s| s.is_available()) {
                    spot.occupied_by = Some(VehicleEntry {
                        vehicle: Vehicle::Car,
                        number_plate,
                        entry_time: Utc::now().timestamp(),
                    });
                    return true;
                } 
                // else if let Some(spot) = self.large_spots.iter_mut().find(|s| s.is_available()) {
                //     spot.occupied_by = Some(VehicleEntry {
                //         vehicle: Vehicle::Car,
                //         number_plate,
                //         entry_time: Utc::now().timestamp(),
                //     });
                //     return true;
                // }
            }
            Vehicle::Van => {
                if let Some(spot) = self.large_spots.iter_mut().find(|s| s.is_available()) {
                    spot.occupied_by = Some(VehicleEntry {
                        vehicle: Vehicle::Van,
                        number_plate,
                        entry_time: Utc::now().timestamp(),
                    });
                    return true;
                }
                // // van needs three spots
                // let available_spots: Vec<_> = self
                //     .large_spots
                //     .iter_mut()
                //     .filter(|s| s.is_available())
                //     .take(3)
                //     .collect();
                // if available_spots.len() == 3 {
                //     for spot in available_spots {
                //         spot.occupied_by = Some(VehicleEntry {
                //             vehicle: Vehicle::Van,
                //             number_plate: number_plate.clone(),
                //             entry_time: Utc::now().timestamp(),
                //         });
                //     }
                //     return true;
                // }
            }
        }
        false
    }

    // exit vehicle
    fn exit(&mut self, vehicle_type: Vehicle, number_plate: &str) -> Option<f32> {
        //  for different vehicle type different fess are there ,     motorcycle = 1, car = 2, van = 3
        let fee_per_sec = match vehicle_type {
            Vehicle::Motorcycle => 1.0,
            Vehicle::Car => 2.0,
            Vehicle::Van => 3.0,
        };
        match vehicle_type {
            Vehicle::Motorcycle => {
                if let Some(spot) = self
                    .motorcycle_spots
                    .iter_mut()
                    .find(|s| s.occupied_by.as_ref().map_or(false, |v| v.number_plate == number_plate))
                {
                    let entry = spot.occupied_by.take().unwrap();
                    let duration = Utc::now().timestamp() - entry.entry_time;
                    let fee = duration as f32 * fee_per_sec * *DURATION_MONEY;
                    return Some(fee);
                }
            }
            Vehicle::Car => {
                if let Some(spot) = self
                    .compact_spots
                    .iter_mut()
                    .find(|s| s.occupied_by.as_ref().map_or(false, |v| v.number_plate == number_plate))
                {
                    let entry = spot.occupied_by.take().unwrap();
                    let duration = Utc::now().timestamp() - entry.entry_time;
                    let fee = duration as f32 * fee_per_sec * *DURATION_MONEY;
                    return Some(fee);
                }
            }
            Vehicle::Van => {
                if let Some(spot) = self
                    .large_spots
                    .iter_mut()
                    .find(|s| s.occupied_by.as_ref().map_or(false, |v| v.number_plate == number_plate))
                {
                    let entry = spot.occupied_by.take().unwrap();
                    let duration = Utc::now().timestamp() - entry.entry_time;
                    let fee = duration as f32 * fee_per_sec * *DURATION_MONEY;
                    return Some(fee);
                }
            }
        }
        None 
    }
}

lazy_static! {
    pub static ref PARKING_TICKET: HashMap<String, (Vehicle, VehicleEntry)> = HashMap::new();
    pub static ref DURATION_MONEY: f32 = 0.10; //  currency unit for every sec
}
fn main() {
    let mut lot = ParkingLot::new(2, 3, 4);
   println!("Initial parking lot state: {:?}", lot);
    println!("Total spots: {}", lot.total_spots());
    println!("Spots remaining: {}", lot.remaining_spots());
    println!("Is full? {}", lot.is_full());
    println!("Is empty? {}", lot.is_empty());
    println!("Motorcycle spots full? {}", lot.motorcycle_full());
    println!("Van spots taken: {}", lot.van_spots_taken());

    lot.park(Vehicle::Motorcycle, "1234".to_string());
    lot.park(Vehicle::Car, "5678".to_string());
    lot.park(Vehicle::Van, "9012".to_string());

    println!("parking lot  :  {:#?}", lot);
    println!("Spots remaining after parking: {}", lot.remaining_spots());
    println!("Van spots taken: {}", lot.van_spots_taken());


    lot.exit(Vehicle::Motorcycle, "1234");
    // lot.exit(Vehicle::Car, "5678");
    // lot.exit(Vehicle::Van, "9012");
    println!("parking lot  :  {:#?}", lot);
    println!("Spots remaining after exiting: {}", lot.remaining_spots());
}


//--------------------------------------------------------------------------------------------------------------------------
//
//
//
//
//
//
//
//

// struct Path<'a> {
//     point_x: &'a i32,
//     point_y: &'a i32,
// }

// // fn main() {
// //     let p_x = 3200;
// //     let p_y = (p_x / 2) as i32;
// //     let maze = Path { point_x: &p_x, point_y: &p_y };
// //     println!("x = {}, y = {}", maze.point_x, maze.point_y);

// // }

// #[derive(Debug)]
// enum TrafficLight{
// Red,
// Yellow,
// Green
// }

// fn next_light(current:TrafficLight)->TrafficLight{
//     match current {
//         TrafficLight::Red => TrafficLight::Green,
//         TrafficLight::Green => TrafficLight::Yellow,
//         TrafficLight::Yellow => TrafficLight::Red,
//     }
// }

// fn main() {
//     // let p_x = 3200;
//     // let temp;
//     // let p_y = {
//     //     temp = 42;
//     //     &temp
//     // };
//     // let maze = Path { point_x: &p_x, point_y: p_y };
//     // println!("x = {}, y = {}", maze.point_x, maze.point_y);

//     let vector = vec![1, 2, 3,3,6,6,7,7,8,5,4,3,2,1,4,5,6,7,8,9,0,3,5,6,8,12,67,34,46,99,44,55,66,77,44,666,32,90,43];

//     let mut hashmap_vec = HashMap::new();
//     for i in vector.iter() {
//         hashmap_vec.entry(format!("{}", i)).or_insert(i);
//     }

//     let data = hashmap_vec.values().map(|&&v| v).collect::<Vec<i32>>();
//     println!("{:?}", hashmap_vec.values().collect::<Vec<&&i32>>());

//    let (vec_data,sum) = get_vec_and_its_sum(data);
//    println!("vec_data = {:?}, sum = {}", vec_data,sum);

//    println!("traffic light : {:?}", next_light(TrafficLight::Red));
//    println!("traffic light : {:?}", next_light(TrafficLight::Green));
//    println!("traffic light : {:?}", next_light(TrafficLight::Yellow));
// }

// fn get_vec_and_its_sum(vec_data: Vec<i32>)->(Vec<i32>,i32){
//     let sum = vec_data.iter().sum();
//     (vec_data.to_vec(),sum)

// }

//

//

//

//

//

//
