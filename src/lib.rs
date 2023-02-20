use serde_derive::{Serialize, Deserialize};

pub const MAX_DATAGRAM_SIZE: usize = 64_000;

// Weatherflow Tempest obs_st JSON record
//
// {
// 	"serial_number": "ST-00055227",
// 	"type": "obs_st",
// 	"hub_sn": "HB-00069665",
// 	"obs":
// 	[
// 	[
// 	1676903673,
// 	0.00,
// 	0.00,
// 	0.00,
// 	0,
// 	3,
// 	999.14,
// 	2.14,
// 	86.22,
// 	24594,
// 	0.44,
// 	205,
// 	0.000000,
// 	0,
// 	0,
// 	0,
// 	2.653,
// 	1
// 	]
// 	],
// 	"firmware_revision": 165
// }

// How ^^ looks to Serde
#[derive(Debug, Serialize, Deserialize)]
pub struct Observation {
	#[serde(rename = "serial_number")]
	pub serial_number: String,
	
	#[serde(rename = "type")]
	pub observation_type: String,
	
	#[serde(rename = "hub_sn")]
	pub hub_sn: String,
	
	#[serde(rename = "obs")]
	pub obs: Vec<Vec<f64>>,
	
	#[serde(rename = "firmware_revision")]
	pub firmware_revision: i64,
}
