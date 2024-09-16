use rocket::{get, launch, routes};

#[get("/")]
async fn hello() -> String {
  "hello world!".to_string()
}


#[launch]
fn rocket() -> _ {
     rocket::build().mount("/hello", routes![hello])
}
