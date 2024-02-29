#[derive(serde::Deserialize)]
pub struct FlightResponse {
    icao24: String,
    first_seen: i32,
    est_departure_airport: Option<String>,
    last_seen: i32,
    est_arrival_airport: Option<String>,
    callsign: Option<String>,
    est_departure_airport_horiz_distance: i32,
    est_departure_airport_vert_distance: i32,
    est_arrival_airport_horiz_distance: i32,
    est_arrival_airport_vert_distance: i32,
    departure_airport_candidates_count: i32,
    arrival_airport_candidates_count: i32,
}
