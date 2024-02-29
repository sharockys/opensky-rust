#[derive(serde::Deserialize)]
pub struct StateResponse {
    icao24: String, //Unique ICAO 24-bit address of the transponder in hex string representation.
    callsign: String, //Callsign of the vehicle (8 chars). Can be null if no callsign has been received
    origin_country: String, //Country name inferred from the ICAO 24-bit address.
    time_position: Option<i64>, //Unix timestamp (seconds) for the last position update. Can be null if no position report was received by OpenSky within the past 15s.
    last_contact: i64, //Unix timestamp (seconds) for the last update in general. This field is updated for any new, valid message
    longitude: Option<f64>, //WGS-84 longitude in decimal degrees. Can be null.
    latitude: Option<f64>, //WGS-84 latitude in decimal degrees. Can be null.
    baro_altitude: Option<f64>, //Barometric altitude in meters. Can be null.
    on_ground: bool, //Boolean value which indicates if the position was retrieved from a surface position report.
    velocity: Option<f64>, //Velocity over ground in m/s. Can be null.
    true_track: Option<f64>, //True track in decimal degrees clockwise from north (north=0°). Can be null.
    vertical_rate: Option<f64>, //Vertical rate in m/s. A positive value indicates that the airplane is climbing, a negative value indicates that it descends. Can be null.
    sensors: Option<Vec<i32>>, //IDs of the receivers which contributed to this state vector. Is null if no filtering for sensor was used in the request.
    geo_altitude: Option<f64>, //Geometric altitude in meters. Can be null.
    squawk: Option<String>,    //The transponder code aka Squawk. Can be null.
    spi: bool,                 //Whether flight status indicates special purpose indicator.
    position_source: Option<i64>, //Origin of this state's position.
    category: Option<i64>,     //Aircraft category.
}

impl std::fmt::Debug for StateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "icao24: {}, callsign: {} origin_country: {} time_position: {:?} last_contact: {} longitude: {:?} latitude: {:?} baro_altitude: {:?} on_ground: {} velocity: {:?} true_track: {:?} vertical_rate: {:?} sensors: {:?} geo_altitude: {:?} squawk: {:?} spi: {} ", self.icao24, self.callsign, self.origin_country, self.time_position, self.last_contact, self.longitude, self.latitude, self.baro_altitude, self.on_ground, self.velocity, self.true_track, self.vertical_rate, self.sensors, self.geo_altitude, self.squawk, self.spi)
    }
}

// constructor
impl StateResponse {
    pub fn new(
        icao24: String,
        callsign: String,
        origin_country: String,
        time_position: Option<i64>,
        last_contact: i64,
        longitude: Option<f64>,
        latitude: Option<f64>,
        baro_altitude: Option<f64>,
        on_ground: bool,
        velocity: Option<f64>,
        true_track: Option<f64>,
        vertical_rate: Option<f64>,
        sensors: Option<Vec<i32>>,
        geo_altitude: Option<f64>,
        squawk: Option<String>,
        spi: bool,
        position_source: Option<i64>,
        category: Option<i64>,
    ) -> StateResponse {
        StateResponse {
            icao24,
            callsign,
            origin_country,
            time_position,
            last_contact,
            longitude,
            latitude,
            baro_altitude,
            on_ground,
            velocity,
            true_track,
            vertical_rate,
            sensors,
            geo_altitude,
            squawk,
            spi,
            position_source,
            category,
        }
    }

    pub fn from_serde_json_value(js: serde_json::Value) -> Self {
        Self {
            icao24: js[0].as_str().unwrap().to_string(),
            callsign: js[1].as_str().unwrap().to_string(),
            origin_country: js[2].as_str().unwrap().to_string(),
            time_position: js[3].as_i64(),
            last_contact: js[4].as_i64().unwrap(),
            longitude: js[5].as_f64(),
            latitude: js[6].as_f64(),
            baro_altitude: js[7].as_f64(),
            on_ground: js[8].as_bool().unwrap(),
            velocity: js[9].as_f64(),
            true_track: js[10].as_f64(),
            vertical_rate: js[11].as_f64(),
            sensors: js[12]
                .as_array()
                .map(|arr| arr.iter().map(|val| val.as_i64().unwrap() as i32).collect()),
            geo_altitude: js[13].as_f64(),
            squawk: js[14].as_str().map(|s| s.to_string()),
            spi: js[15].as_bool().unwrap(),
            position_source: js[16].as_i64(),
            category: js[17].as_i64(),
        }
    }
}

// +-------+-------------------+---------+------------------------------------------------------------------+
// | Index | Property          | Type    | Description                                                      |
// +=======+===================+=========+==================================================================+
// | 0     | *icao24*          | string  | Unique ICAO 24-bit address of the transponder in hex string      |
// |       |                   |         | representation.                                                  |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 1     | *callsign*        | string  | Callsign of the vehicle (8 chars). Can be null if no callsign    |
// |       |                   |         | has been received.                                               |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 2     | *origin_country*  | string  | Country name inferred from the ICAO 24-bit address.              |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 3     | *time_position*   | int     | Unix timestamp (seconds) for the last position update. Can be    |
// |       |                   |         | null if no position report was received by OpenSky within the    |
// |       |                   |         | past 15s.                                                        |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 4     | *last_contact*    | int     | Unix timestamp (seconds) for the last update in general. This    |
// |       |                   |         | field is updated for any new, valid message received from the    |
// |       |                   |         | transponder.                                                     |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 5     | *longitude*       | float   | WGS-84 longitude in decimal degrees. Can be null.                |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 6     | *latitude*        | float   | WGS-84 latitude in decimal degrees. Can be null.                 |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 7     | *baro_altitude*   | float   | Barometric altitude in meters. Can be null.                      |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 8     | *on_ground*       | boolean | Boolean value which indicates if the position was retrieved from |
// |       |                   |         | a surface position report.                                       |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 9     | *velocity*        | float   | Velocity over ground in m/s. Can be null.                        |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 10    | *true_track*      | float   | True track in decimal degrees clockwise from north (north=0°).   |
// |       |                   |         | Can be null.                                                     |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 11    | *vertical_rate*   | float   | Vertical rate in m/s. A positive value indicates that the        |
// |       |                   |         | airplane is climbing, a negative value indicates that it         |
// |       |                   |         | descends. Can be null.                                           |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 12    | *sensors*         | int[]   | IDs of the receivers which contributed to this state vector.     |
// |       |                   |         | Is null if no filtering for sensor was used in the request.      |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 13    | *geo_altitude*    | float   | Geometric altitude in meters. Can be null.                       |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 14    | *squawk*          | string  | The transponder code aka Squawk. Can be null.                    |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 15    | *spi*             | boolean | Whether flight status indicates special purpose indicator.       |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 16    | *position_source* | int     | Origin of this state's position.                                 |
// |       |                   |         |                                                                  |
// |       |                   |         | * 0 = ADS-B                                                      |
// |       |                   |         | * 1 = ASTERIX                                                    |
// |       |                   |         | * 2 = MLAT                                                       |
// |       |                   |         | * 3 = FLARM                                                      |
// +-------+-------------------+---------+------------------------------------------------------------------+
// | 17    | *category*        | int     | Aircraft category.                                               |
// |       |                   |         |                                                                  |
// |       |                   |         | * 0 = No information at all                                      |
// |       |                   |         | * 1 = No ADS-B Emitter Category Information                      |
// |       |                   |         | * 2 = Light (< 15500 lbs)                                        |
// |       |                   |         | * 3 = Small (15500 to 75000 lbs)                                 |
// |       |                   |         | * 4 = Large (75000 to 300000 lbs)                                |
// |       |                   |         | * 5 = High Vortex Large (aircraft such as B-757)                 |
// |       |                   |         | * 6 = Heavy (> 300000 lbs)                                       |
// |       |                   |         | * 7 = High Performance (> 5g acceleration and 400 kts)           |
// |       |                   |         | * 8 = Rotorcraft                                                 |
// |       |                   |         | * 9 = Glider / sailplane                                         |
// |       |                   |         | * 10 = Lighter-than-air                                          |
// |       |                   |         | * 11 = Parachutist / Skydiver                                    |
// |       |                   |         | * 12 = Ultralight / hang-glider / paraglider                     |
// |       |                   |         | * 13 = Reserved                                                  |
// |       |                   |         | * 14 = Unmanned Aerial Vehicle                                   |
// |       |                   |         | * 15 = Space / Trans-atmospheric vehicle                         |
// |       |                   |         | * 16 = Surface Vehicle – Emergency Vehicle                       |
// |       |                   |         | * 17 = Surface Vehicle – Service Vehicle                         |
// |       |                   |         | * 18 = Point Obstacle (includes tethered balloons)               |
// |       |                   |         | * 19 = Cluster Obstacle                                          |
// |       |                   |         | * 20 = Line Obstacle                                             |
// +-------+-------------------+---------+------------------------------------------------------------------+
