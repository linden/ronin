use rocket::http;
use rocket::request;
use rocket::outcome;
use rocket::State;
use r2d2;
use r2d2_redis::RedisConnectionManager;

const REDIS_ADDRESS: &'static str = "redis://localhost:6379";

// pool initialisation.
// called on Ronin boot for API instance + events handler
// stores a db connection pool as a rocket managed state.
pub fn pool() -> Pool {
    let manager = RedisConnectionManager::new(REDIS_ADDRESS).expect("connection manager");
    let redis_config = Default::default();

    r2d2::Pool::new(redis_config, manager).expect("db pool")
}

// Rocket guard type: a wrapper around an r2d2 pool.
// In conjunction with
//   impl<'a, 'r> request::FromRequest<'a, 'r> for RedisConnection`
// which is super confusing, it also allows code like:
//   #[post("/<item>")]
//   fn create(item: &RawStr, connection: RedisConnection) -> ...
pub struct RedisConnection(pub r2d2::PooledConnection<RedisConnectionManager>);

// alias to the type for a pool of redis connections.
type Pool = r2d2::Pool<RedisConnectionManager>;

// retrieving a single connection from the database pool.
impl<'a, 'r> request::FromRequest<'a, 'r> for RedisConnection {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<RedisConnection, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(RedisConnection(conn)),
            Err(_) => Outcome::Failure((http::Status::ServiceUnavailable, ())),
        }
    }
}
