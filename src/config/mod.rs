use std::env;

pub fn get_web_service_config() -> (String, String) {
    let host = match env::var("HOST") {
        Ok(host) => host,
        Err(_) => {
            println!("You didn't set HOST variable, using default value 127.0.0.1",);
            "127.0.0.1".to_string()
        }
    };
    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => {
            println!("You didn't set PORT variable, using default value 8080",);
            "8080".to_string()
        }
    };
    (host, port)
}

