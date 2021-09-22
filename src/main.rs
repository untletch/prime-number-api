#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::Serialize;
use std::time::Instant;

#[derive(Serialize)]
struct NumberResponse {
    number: u128,
    is_prime_number: bool,
    execution_time_in_microsecs: u128,
    execution_time_in_seconds: u64,
}

#[get("/")]
fn index() -> &'static str {
    "Test whether digit is prime or not.\nDigit is a u128.\nFor example: http:localhost:8000/isPrime/951224568960583(15 digit prime)"
}

#[get("/isPrime/<number>")]
async fn get_is_prime(number: u128) -> Json<NumberResponse> {
    let now = Instant::now();
    Json(NumberResponse {
        number,
        is_prime_number: is_prime(number),
        execution_time_in_microsecs: now.elapsed().as_micros(),
        execution_time_in_seconds: now.elapsed().as_secs(),
    })
}

fn is_prime(n: u128) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n == 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5u128;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_is_prime])
}
