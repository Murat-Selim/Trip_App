use std::io;

fn main() {
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("failed to read");
    let id: i32 = id.trim().parse().expect("failed to convert");

    let mut destination = String::new();
    io::stdin().read_line(&mut destination).expect("failed to read");

    let mut start_date = String::new();
    io::stdin().read_line(&mut start_date).expect("failed to read");

    let mut end_date = String::new();
    io::stdin().read_line(&mut end_date).expect("failed to read");
    
    let new_trip = Trip::new(id, destination, start_date, end_date);
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


