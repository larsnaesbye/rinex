//! `RINEX` files type description 
use thiserror::Error;
use std::str::FromStr;

/// Describes all known `RINEX` file types
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Type {
    /// Describes Observation Data (OBS),
    /// Phase & Pseudo range measurements
    ObservationData, 
    /// Describes Navigation Message (NAV)
    /// Ephemeride file
    NavigationMessage,
    /// Describes Meteorological data (Meteo)
    MeteorologicalData,
}

#[derive(Error, Debug)]
/// `Type` related errors
pub enum TypeError {
    #[error("Unknown RINEX type identifier \"{0}\"")]
    UnknownType(String),
}

impl Default for Type {
    /// Builds a default `Type`
    fn default() -> Type { Type::ObservationData }
}

impl Type {
    /// Converts `Self` to str
    pub fn to_str (&self) -> &str {
        match *self {
            Type::ObservationData => "ObservationData",
            Type::NavigationMessage => "NavigationMessage",
            Type::MeteorologicalData => "MeteorologicalData",
        }
    }
    /// Converts `Self` to string
    pub fn to_string (&self) -> String { String::from(self.to_str()) }
}

impl std::str::FromStr for Type {
    type Err = TypeError;
    fn from_str (s: &str) -> Result<Self, Self::Err> {
        if s.eq("NAVIGATION DATA") {
            Ok(Type::NavigationMessage)
        } else if s.contains("NAV DATA") {
            Ok(Type::NavigationMessage)
        } else if s.eq("OBSERVATION DATA") {
            Ok(Type::ObservationData)
        } else if s.eq("METEOROLOGICAL DATA") {
            Ok(Type::MeteorologicalData)
        } else {
            Err(TypeError::UnknownType(String::from(s)))
        }
    }
}