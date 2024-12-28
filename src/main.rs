use std::process::ExitCode;

use film::Film;

mod film;
mod jellyfin;

fn main() -> ExitCode {
    let jf_client = jellyfin::init();
    let jf_user = match jellyfin::login(&jf_client) {
        Ok(user) => user,
        Err(error) => {
            eprintln!("Failed to login to JellyFin: {}", error);
            return ExitCode::from(3);
        }
    };
    let jf_films = match jellyfin::get_all_films(&jf_client, &jf_user) {
        Ok(films) => films,
        Err(error) => {
            eprintln!("Failed to get JellyFin films: {}", error);
            return ExitCode::from(4);
        }
    };

    let duplicate_films: Vec<&Film> = jf_films
        .iter()
        .filter(|jf_film| {
            jf_films.iter().any(|other_film| {
                jf_film.id > other_film.id
                    && jf_film.title == other_film.title
                    && jf_film.year == other_film.year
            })
        })
        .collect();

    println!("\n{} duplicate films found:", duplicate_films.len());
    for film in &duplicate_films {
        println!("{} ({})", film.title, film.year);
    }

    ExitCode::SUCCESS
}
