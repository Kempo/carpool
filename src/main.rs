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

fn is_car_full(driver_name: &'static str, space: usize, drivers: &Vec<Person<Driver>>) -> bool {
    let found = drivers.iter().find(|candidate| candidate.name == driver_name);

    match found {
        Some(Person { variant: Some(driver), .. }) => driver.total_space == space,
        _ => true // default
    }
}

fn rider_is_taken(rider_name: &'static str, assignments: &HashMap<&'static str, Vec<&'static str>>) -> bool {
    assignments.values().any(|list| list.contains(&rider_name))
}

fn strongest_match(rider_name: &'static str, driver_name: &'static str, assignments: &HashMap<&'static str, Vec<&'static str>>, drivers: &Vec<Person<Driver>>) -> bool {
    // since we're iterating over the rider's preference, that'll already be covered.
    // we know: rider prefers this driver the most.

    let option = drivers.iter().find(|driver| driver.name == driver_name);

    if let Some(driver) = option {
        let first_available_pref = driver.preferences.iter().find(|r| !rider_is_taken(r, assignments));

        return match first_available_pref {
            Some(name) => *name == rider_name,
            None => false
        };
    }
    // get driver's first available pref
    // return whether driver also prefers this rider over any others that *aren't* taken

    false
}

fn main() {
    println!("Starting...");

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
                    let car = assignments.entry(pref_name).or_insert_with(|| vec![]);

                    if !is_car_full(pref_name, car.len(), &drivers) && strongest_match(rider.name, pref_name, &assignments, &drivers) {
                        if let Some(car) = assignments.get_mut(pref_name) {
                            println!("Assigning rider {} to driver {}", rider.name, pref_name);
                            car.push(rider.name);
                            
                            filled += 1;
                        }
                    }
                }
            }
        }
    }
}