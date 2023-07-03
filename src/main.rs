fn main() {
    let new_trip = Trip::new(1, "Istanbul".to_string(), "2023-07-01".to_string(), "2023-07-31".to_string());
    let trip_ref = &new_trip;

    println!("Trip to {}:", trip_ref.destination);

}

struct Trip {
    id: i32,
    destination: String,
    start_date: String,
    end_date: String,
}

impl Trip {
    fn new(id: i32, destination: String, start_date: String, end_date: String) -> Self {
        Self {
            id,
            destination,
            start_date,
            end_date,
        }
    }
}


