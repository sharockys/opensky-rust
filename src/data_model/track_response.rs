pub mod track_response {
    #[derive(serde::Deserialize)]
    pub struct Waypoint {
        lat: f64,  //Latitude in decimal degrees
        lon: f64,  //Longitude in decimal degrees
        alt: i64,  //Altitude in feet
        time: i64, //Unix timestamp
    }
    #[derive(serde::Deserialize)]
    pub struct TrackResponse {
        icao24: String,      //Unique ICAO 24-bit
        start_time: i64,     //Time of the first waypoint in seconds since epoch (Unix time).
        end_time: i64,       //Time of the last waypoint in seconds since epoch (Unix time).
        callsign: String,    //Callsign (8 characters) that holds for the whole track. Can be null.
        path: Vec<Waypoint>, //Waypoints of the trajectory (description below).
    }
}
// The response is a JSON object with the following properties:

// Property	Type	Description
// icao24	string	Unique ICAO 24-bit address of the transponder in lower case hex string representation.
// startTime	integer	Time of the first waypoint in seconds since epoch (Unix time).
// endTime	integer	Time of the last waypoint in seconds since epoch (Unix time).
// calllsign	string	Callsign (8 characters) that holds for the whole track. Can be null.
// path	array	Waypoints of the trajectory (description below).
