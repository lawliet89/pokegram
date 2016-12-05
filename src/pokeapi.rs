use ::rest;

#[derive(RustcDecodable, Eq, PartialEq, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct Type {
    id: i32,
    name: String,
    damage_relations: DamageRelations,
    names: Name
}

#[derive(RustcDecodable, Eq, PartialEq, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct DamageRelations {
    no_damage_to: Vec<NamedApiResource>,
    half_damage_to: Vec<NamedApiResource>,
    double_damage_to: Vec<NamedApiResource>,
    no_damage_from: Vec<NamedApiResource>,
    half_damage_from: Vec<NamedApiResource>,
    double_damage_from: Vec<NamedApiResource>
}

// Common Mode
#[derive(RustcDecodable, Eq, PartialEq, Clone, Debug)]
pub struct NamedApiResource {
    name: String,
    url: String
}

#[derive(RustcDecodable, Eq, PartialEq, Clone, Debug)]
pub struct Name {
    name: String,
    language: NamedApiResource
}

pub struct PokeApi {
    endpoint: String
}


const VERSION:  &'static str = "v2";
const TYPE:  &'static str = "type";
const USER_AGENT: &'static str = "github.com/lawliet89/pokegram";

impl PokeApi {
    pub fn new(endpoint: &str) -> PokeApi {
        PokeApi {
            endpoint: endpoint.to_owned()
        }
    }

    fn make_url(&self, endpoint: &str) -> String {
        format!("{}/{}/{}", self.endpoint, VERSION, endpoint)
    }

    pub fn poke_type(&self, type_name: &str) -> Result<Type, String> {
        let mut headers = rest::Headers::new();
        headers.add_user_agent(USER_AGENT);

        let url = self.make_url(&format!("{}/{}", TYPE, type_name));
        match rest::get::<Type>(&url, &headers.headers) {
            Ok(pokemon_type) => Ok(pokemon_type),
            Err(err) => Err(format!("Error getting type {}", err))
        }
    }
}
