use std::collections::HashMap;

trait Participant {
    fn new(&self, name: &'static str, preferences: Vec<&'static str>, optional_space: Option<usize>) -> Result<Self, &'static str>
        where Self : Sized;
    
    fn get_name(&self) -> &'static str;
}

struct Driver {
    name: &'static str,
    preferences: Vec<&'static str>,
    car_capacity: usize
}

impl Participant for Driver {
    fn new(&self, name: &'static str, preferences: Vec<&'static str>, space: Option<usize>) -> Result<Driver, &'static str> {
        return match space {
            Some(size) => Ok(Driver {
                name, 
                preferences,
                car_capacity: size
            }),
            None => Err("Cannot create Driver without a provided car space.")
        };
    }

    fn get_name(&self) -> &'static str {
        self.name
    }
}

struct Rider {
    name: &'static str,
    preferences: Vec<&'static str>
}

fn main() {

    let drivers: Vec<Driver> = vec![];
    let riders: Vec<Rider> = vec![];

    // <driver_name, [rider1, rider2]>
    let mut assignments: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
}

// Initialize with preferences for both riders and drivers
// Find strongest preferences from both (read-only)
// Mutate ONLY the assignments hashmap