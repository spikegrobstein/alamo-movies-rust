use super::cinema::Cinema;
use super::film::Film;

use crate::alamo_movies::market::Market;

pub enum Format {
    Text,
    Json,
}

/// given a vector of films, print them out accordingly
pub fn list_films(films: &Vec<Film>, format: &Format) {
    match format {
        Format::Json => json_list_films(films),
        Format::Text => {
            for film in films.iter() {
                film_info(film, format);
            }
        },
    }
}

pub fn list_markets(markets: &Vec<Market>, format: &Format) {
    match format {
        Format::Json => json_list_markets(markets),
        Format::Text => {
            for market in markets {
                market_info(market, format);
            }
        },
    }
}

pub fn film_info(film: &Film, _format: &Format) {
    println!("{}", film.name)
}

pub fn json_list_films(films: &Vec<Film>) {
    match serde_json::to_string(films) {
        Ok(json) => println!("{}", json),
        _ => panic!("whoops"),
    }
}

pub fn market_info(market: &Market, _format: &Format) {
    let id = &market.id;
    let slug = &market.slug;
    let name = &market.name;

    println!("{id} [{slug}] {name}")
}

pub fn cinema_info(cinema: &Cinema, _format: &Format) {
    println!("{cinema_id} [{cinema_slug}] {cinema_name} ({market})",
        cinema_id = cinema.id,
        cinema_slug = cinema.slug,
        cinema_name = cinema.name,
        market = cinema.market.name,
    );
}

pub fn json_cinema_info(cinema: &Cinema) {
    match serde_json::to_string(cinema) {
        Ok(json) => println!("{}", json),
        _ => panic!("whoops"),
    }
}

pub fn json_list_markets(markets: &Vec<Market>) {
    match serde_json::to_string(markets) {
        Ok(json) => println!("{json}"),
        _ => panic!("whoops."),
    }
}

pub fn json_list_cinemas(cinemas: &Vec<Cinema>) {
    match serde_json::to_string(cinemas) {
        Ok(json) => println!("{}", json),
        _ => panic!("whoops"),
    }
}

