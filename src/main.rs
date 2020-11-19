use std::collections::HashMap;

struct Person<T> {
    name: &'static str,
    preferences: Vec<&'static str>,
    variant: Option<T>
}

impl Person<Driver> {
    fn new(name: &'static str, preferences: Vec<&'static str>, total_space: usize) -> Self {
        Self {
            name,
            preferences,
            variant: Some(Driver {
                total_space
            })
        }
    }
}

impl Person<Rider> {
    fn new(name: &'static str, preferences: Vec<&'static str>) -> Self {
        Self {
            name, 
            preferences,
            variant: None
        }
    }
}

struct Driver {
    total_space: usize
}

struct Rider;

fn is_car_full(driver_name: &'static str, seats: &Vec<&'static str>, drivers: &Vec<Person<Driver>>) -> bool {
    let found = drivers.iter().find(|candidate| candidate.name == driver_name);

    return match found {
        Some(Person { variant: Some(driver), .. }) => driver.total_space == seats.len(),
        _ => true // default
    };
}

// loop through `assignments` ref
fn rider_is_taken(rider_name: &'static str, assignments: &HashMap<&'static str, Vec<&'static str>>) -> bool {
    true
}

fn strongest_match(rider_name: &'static str, driver_name: &'static str, assignments: &HashMap<&'static str, Vec<&'static str>>) -> bool {
    true
}

fn main() {

    let drivers: Vec<Person<Driver>> = vec![
        Person::<Driver>::new("Eric", vec!["Tom", "Max"], 3),
        Person::<Driver>::new("Charles", vec!["Max", "Tom"], 3)
    ];

    let riders: Vec<Person<Rider>> = vec![
        Person::<Rider>::new("Tom", vec!["Eric", "Charles"]),
        Person::<Rider>::new("Max", vec!["Charles", "Eric"]),
    ];

    let total_riders = riders.len();
    let mut filled: usize = 0;

    // <driver_name, [rider1, rider2]>
    let mut assignments: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

    while filled < total_riders {
        for rider in &riders {
            if !rider_is_taken(rider.name, &assignments) {
                for pref_name in &rider.preferences {
                    match assignments.get(pref_name) {
                        Some(car_val) => {
                            if !is_car_full(pref_name, car_val, &drivers) && strongest_match(rider.name, pref_name, &assignments) {
                                if let Some(car) = assignments.get_mut(pref_name) {
                                    car.push(rider.name);
                                    filled += 1;
                                }
                            }
                        },
                        None => { 
                            println!("Cannot identify driver. Skipping...");
                        }
                    };
                }
            }
        }
    }
}

// Initialize with preferences for both riders and drivers
// Find strongest preferences from both (read-only)
// Mutate ONLY the assignments hashmap