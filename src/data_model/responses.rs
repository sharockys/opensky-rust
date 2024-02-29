use super::states_responses::StateResponse;
use std::fmt;

#[derive(serde::Deserialize)]
pub struct StateAllResponse {
    time: i64,
    states: Vec<StateResponse>,
}

impl StateAllResponse {
    pub fn from_string(response_body: String) -> Result<Self, serde_json::Error> {
        let response: serde_json::Value = serde_json::from_str(&response_body)?;
        let states: &Vec<serde_json::Value> = response["states"].as_array().unwrap();
        let states = states
            .iter()
            .map(|state| StateResponse::from_serde_json_value(state.clone()))
            .collect();
        let time = response["time"].as_i64().unwrap();
        Ok(StateAllResponse { time, states })
    }
}

impl fmt::Debug for StateAllResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "time: {:?} state_0: {:?}", self.time, self.states[0])
    }
}
