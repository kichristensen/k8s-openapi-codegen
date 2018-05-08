use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Info {
	pub title: String,
	pub version: String,
}
