lazy_static::lazy_static! {
    pub static ref JWT_SECRET: String = {
        if let Ok(secret) = std::env::var("JWT_SECRET") {
             secret
        } else {
            #[cfg(feature="local_env")]
            return dotenv_codegen::dotenv!("JWT_SECRET").to_string();
            #[cfg(not(feature="local_cdn"))]
            "".to_string()
        }
    };
}