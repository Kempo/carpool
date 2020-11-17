use std::collections::HashMap;

struct Details {
    _address: &'static str, // String's are allocated on the heap (unknown size of String at compile-time)
    preferences: Vec<&'static str> // all person ids
}

struct Rider {
    personal: Details,
    is_taken: bool
}

struct Driver {
    personal: Details,
    car: Car,
}

struct Car {
    passengers: Vec<Rider> // initialize with `with_capacity`
}

impl Car {
    fn num_available_seats(&self) -> usize {
        self.passengers.len() // length is number of filled
    }

    fn total_seats(&self) -> usize {
        self.passengers.capacity() // capacity is total length
    }
}

impl Driver {
    fn add_rider(&mut self, rider: Rider) {
        // check for panic?
        self.car.passengers.push(rider);
    }

    fn is_full(&self) -> bool {
        self.car.num_available_seats() >= self.car.total_seats()
    }

    fn new(preferences: Vec<&'static str>, space: u8) -> Driver {
        Driver {
            personal: Details {
                _address: "some-value",
                preferences
            },
            car: Car {
                passengers: Vec::<Rider>::with_capacity(space as usize) // hm?
            }
        }
    }
}

impl Rider {
    fn new(preferences: Vec<&'static str>) -> Rider {
        Rider {
            personal: Details {
                _address: "other-value",
                preferences,
            },
            is_taken: false
        }
    }

    fn prefers_strongest(&self, selected_driver_name: &str, statuses: &HashMap<&str, Driver>) -> bool {

        // loop through the top preferences of this Rider
        // get first preference that IS available
        // if that pref = driver, return true

        let mut first_available: Option<&str> = None;

        for pref in self.personal.preferences.iter() {
            let slice = &pref[..];
            if let Some(driver) = statuses.get(slice) {
                if !driver.is_full() {
                    first_available = Some(slice);
                    break;
                }
            }else { println!("Invalid driver name found."); }
        }

        match first_available {
            Some(name) => selected_driver_name == name,
            _ => false
        }
    }
}

fn main() {
    let mut drivers: HashMap<&str, Driver> = HashMap::new();

    drivers.insert("Nolan", Driver::new(vec!["Tim", "Martin"], 3));
    drivers.insert("Eric", Driver::new(vec!["Martin", "Tim"], 3));

    let mut riders: HashMap<&str, Rider> = HashMap::new();

    riders.insert("Tim", Rider::new(vec!["Nolan", "Eric"]));
    riders.insert("Martin", Rider::new(vec!["Eric", "Nolan"]));

    let mut assignments: HashMap<&str, Vec<&str>> = HashMap::new();

    let mut filled: usize = 0;

    // assume there is enough space for all riders
    while filled <= riders.len() {

        for (driver_name, driver) in &drivers {
            if !driver.is_full() {
                for pref_rider_name in &driver.personal.preferences {
                    if let Some(rider)  = riders.get_mut(pref_rider_name) {
                        if !rider.is_taken && rider.prefers_strongest(driver_name, &drivers) {
                            rider.is_taken = true;

                            // increment space for driver?
                            if assignments.contains_key(driver_name) {
                                match assignments.get_mut(driver_name) {
                                    Some(prefs) => {
                                        prefs.push(pref_rider_name);
                                    },
                                    _ => ()
                                };
                            }else {
                                assignments.insert(driver_name, vec![pref_rider_name]);
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Whoop whoop!");
}
