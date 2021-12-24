#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate figment;
extern crate num_cpus;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    // Use Rocket's default `Figment`, but allow values from `MyApp.toml`
    // and `MY_APP_` prefixed environment variables to supersede its values.
    use rocket::config::{Config, Environment};

    // we want to share system resources between the API and the Events handler
    // events gets a single thread, and the API uses the rest
    let api_workers = num_cpus::get()-1;
    let event_workers = 1;

    // create thread for events handler
    std::thread::spawn(move || {
        let config = Config::build(Environment::Staging)
            .address("0.0.0.0")
            .port(3700)
            .workers(event_workers)
            //.ident("Ronin", true)
            .finalize()
            .unwrap();

        rocket::custom(config)
            .mount("/", routes![hello])
            .launch();
    });

    // main thread for API
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(3000)
        .workers(api_workers.try_into().unwrap())
        //.ident("Ronin", true)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![hello])
        .launch();
}

