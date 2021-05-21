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

pub fn get_mongodb_config() -> (String, String) {
    let host = match env::var("MONGO_HOST") {
        Ok(host) => host,
        Err(_) => {
            println!("You didn't set MONGO_HOST variable, using default value 127.0.0.1",);
            "127.0.0.1".to_string()
        }
    };
    let port = match env::var("MONGO_PORT") {
        Ok(port) => port,
        Err(_) => {
            println!("You didn't set MONGO_PORT variable, using default value 27017",);
            "27017".to_string()
        }
    };

    let user = match env::var("MONGO_USER") {
        Ok(user) => user,
        Err(_) => {
            println!("You didn't set MONGO_USER variable, using default value mongoadmin",);
            "mongoadmin".to_string()
        }
    };
    let password = match env::var("MONGO_PASSWORD") {
        Ok(password) => password,
        Err(_) => {
            println!("You didn't set MONGO_PASSWORD variable, using default value tiny-url-shortener",);
            "tiny-url-shortener".to_string()
        }
    };

    let uri = format!("mongodb://{0}:{1}@{2}:{3}", user, password, host, port);

    let db_name = match env::var("MONGO_DB_NAME") {
        Ok(db_name) => db_name,
        Err(_) => {
            println!("You didn't set MONGO_DB_NAME variable, using default value tiny-url-shortener",);
            "tiny-url-shortener".to_string()
        }
    };
    (uri, db_name)
}

