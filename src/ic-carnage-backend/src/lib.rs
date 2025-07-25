// use ic_cdk::api::caller;
// use ic_cdk_macros::{query, update};
use candid::Principal;
use std::cell::RefCell;
//use candid::{CandidType, Deserialize};

//car struct
#[derive(Clone, Debug, candid::CandidType, candid::Deserialize)]
pub struct Car {
    pub id: u8,
    pub name: String,
    pub acceleration: u8,
    pub top_speed: u8,
    pub handling: u8,
    pub armor: u8
}

//static list of cars

thread_local! {
    static CARS: Vec<Car> = vec![
        Car {
            id: 0,
            name: "Bolt".to_string(),
            acceleration: 9,
            top_speed: 10,
            handling: 6,
            armor: 3,
        },
        Car {
            id: 1,
            name: "Tank".to_string(),
            acceleration: 3,
            top_speed: 6,
            handling: 5,
            armor: 10,
        },
        Car {
            id: 2,
            name: "Stinger".to_string(),
            acceleration: 7,
            top_speed: 9,
            handling: 7,
            armor: 5,
        },
        Car {
            id: 3,
            name: "Slick".to_string(),
            acceleration: 6,
            top_speed: 7,
            handling: 9,
            armor: 4,
        },
    ]
}

//Race result log

type RaceLog = Vec<(Principal, u8, u8)>; //(player, car_id, place)
thread_local! {
    static RACE_RESULTS: RefCell<RaceLog> = RefCell::new(Vec::new());
}


#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {name}! Welcome to IC Carnage!" )
}

#[ic_cdk::query]
fn get_available_cars() -> Vec<Car> {
    CARS.with(|cars| cars.clone())
}

#[ic_cdk::update]
fn record_race_result(player_id: Principal, car_id: u8, place:u8) {
    RACE_RESULTS.with(|log| {
        log.borrow_mut().push((player_id, car_id, place))
    })
}