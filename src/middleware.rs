use actix_jwt_session::*;
use redis::RedisError;
use redis_async_pool::{deadpool::managed::Pool, RedisConnection};
use std::sync::Arc;

use crate::auth::AppClaims;

pub fn prepare_jwt(redis_pool: Pool<RedisConnection, RedisError>) -> (SessionStorage, SessionMiddlewareFactory<AppClaims>, JwtTtl, RefreshTtl) {
    let keys = JwtSigningKeys::load_or_create();

    let (storage, factory) = SessionMiddlewareFactory::<AppClaims>::build(
        Arc::new(keys.encoding_key), 
        Arc::new(keys.decoding_key), 
        Algorithm::HS256
    )
    .with_redis_pool(redis_pool)
    .with_jwt_header(JWT_HEADER_NAME)
    .with_jwt_cookie(JWT_COOKIE_NAME)
    .finish();

    let jwt_ttl = JwtTtl(Duration::days(14));
    let refresh_ttl = RefreshTtl(Duration::days(3*31));

    return (storage, factory, jwt_ttl, refresh_ttl);
}