use rocket::Error;

#[derive(Clone)]
pub struct Toot {
    pub embed: String,
}

pub fn get_toot_embed_code(url: String) -> Result<Toot, Error> {
    return Ok(Toot {
        embed: "root toot".to_string(),
    });
}

pub fn get_children(url: String) -> Result<Vec<Toot>, Error> {
    return Ok(vec![
        Toot {
            embed: "second toot".to_string(),
        },
        Toot {
            embed: "third toot".to_string(),
        },
    ]);
}
