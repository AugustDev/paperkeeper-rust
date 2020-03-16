use rocket::State;

#[get("/<id>")]
pub fn get_user(id: Option<String>) -> String {
    match id {
        Some(v) => format!("Hello, {}!", v),
        None => format!("Hello, missing xtds n!"),
    }
}
