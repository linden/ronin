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

    let cpu_cores = num_cpus::get();

    if cpu_cores == 1 { panic!("You need at least 2 CPU cores to run Ronin!"); }

    // we want to share system resources between the API and the Events handler
    // events gets a single thread, and the API uses the rest
    let api_workers = cpu_cores-1;
    let event_workers = 1;
    
    println!("Booting Ronin with {} API workers and {} event worker", api_workers, event_workers);

    // create thread for events handler
    let events_handler = std::thread::spawn(move || {
        // TODO: have profiles for dev, staging, prod
        // TODO: support custom profiles
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
        // TODO: have profiles for dev, staging, prod
        // TODO: support custom profiles
        .address("0.0.0.0")
        .port(3000)
        .workers(api_workers.try_into().unwrap())
        //.ident("Ronin", true)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![hello])
        .launch();

    // wait for eventsHandler to finish before exiting
    events_handler.join().unwrap();
}

