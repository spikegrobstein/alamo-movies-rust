use std::fs;
use std::error::Error;
use std::path::Path;

use reqwest;
use regex::Regex;

use super::market::Market;
use super::film::Film;
use super::db;
use super::error::{InvalidCinemaData};

use crate::alamo_movies::market::MarketApiResponse;
use crate::alamo_movies::presentation::Presentation;

use serde::{Serialize, Deserialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Cinema {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub market: Market,
}

impl Cinema {
    pub fn list() -> [Cinema; 0] {
       [
       ]
    }

    pub fn to_cinema_id(cinema_id: &str) -> Option<String> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^\d+$") // only digits
                    .unwrap();
        }

        // if it's a match, then return it back
        if RE.is_match(cinema_id) {
            return Some(String::from(cinema_id));
        }

        // otherwise we need to look the thing up
        let cinemas = Cinema::list();

        let cinema_id =
            cinemas.iter()
                .find(|c| c.slug == cinema_id);
        
        cinema_id.map(|c| c.id.clone())
    }

    pub fn from_calendar_file(path: &Path) -> Result<(Market, Vec<Presentation>), Box<dyn Error>> {
        unimplemented!("not implemented");
        // let contents = fs::read_to_string(path.to_str().unwrap())?;
        // Cinema::from_calendar_data(&contents)
    }

    pub fn from_calendar_data(data: &str) -> Result<(Cinema, Vec<Film>), Box<dyn Error>> {
        unimplemented!("not implemented");
        // let resp: MarketApiResponse = serde_json::from_str(data)?;

        // Ok(resp.data.market.first(), resp.data.presentations)
    }

    pub fn from_calendar_json(json: &serde_json::Value) -> Result<(Cinema, Vec<Film>), Box<dyn Error>> {
        unimplemented!("not implemented");
        // let name = cinema_name_from(json).unwrap();
        // let id = cinema_id_from(json).unwrap();
        // let slug = cinema_slug_from(json).unwrap();

        // let market = Market::from_calendar_json(json)?;

        // Ok((Cinema {
            // id: id.to_string(),
            // name: name.to_string(),
            // slug: slug.to_string(),
            // market,
            // },
            // Film::films_from_calendar(json)?,
        // ))
    }

    pub fn get_calendar_data(slug: &str) -> Result<String, Box<dyn Error>>  {
        let url = format!("https://drafthouse.com/s/mother/v2/schedule/market/{slug}");

        println!("requesting file...");

        let body = reqwest::get(&url)?
            .text()?;

        Ok(body)
    }

    pub fn sync_file(cinema_id: &str) -> Result<(Self, Vec<Film>), Box<dyn Error>> {
        let body = Self::get_calendar_data(cinema_id)?;

        // validate:
        let result = Self::from_calendar_data(&body)?;

        db::write_calendar_file(cinema_id, &body)?;

        Ok(result)
    }
}

fn cinema_name_from(data: &serde_json::Value) -> Option<&str> {
    data["CinemaName"].as_str()
}

fn cinema_id_from(data: &serde_json::Value) -> Option<&str> {
    data["CinemaId"].as_str()
}

fn cinema_slug_from(data: &serde_json::Value) -> Option<&str> {
    data["CinemaSlug"].as_str()
}

