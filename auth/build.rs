use min_auth_common::{
    config::auth::{AuthConfig, ExposeConfig, RedisConfig, SecurityConfig},
    DynError,
};

fn gen_config() -> Result<(), DynError> {
    let config = AuthConfig {
        expose: ExposeConfig {
            sockets: vec![
                "127.0.0.1:50080".to_string(),
                "127.0.0.1:50081".to_string(),
                "127.0.0.1:50082".to_string(),
                "127.0.0.1:50083".to_string(),
            ],
        },
        security: SecurityConfig {
            password_secret: "secret".to_string(),
        },
        redis: RedisConfig {
            uri: "redis://127.0.0.1/0".to_string(),
        },
    };
    config.save("etc/min-auth/auth.toml.example")?;

    Ok(())
}

fn main() -> Result<(), DynError> {
    gen_config()?;
    Ok(())
}
