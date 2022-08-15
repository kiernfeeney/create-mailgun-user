use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api_key = String::new();
    let mut domain = String::new();
    let mut user = String::new();
    let mut password_length = String::new();

    println!("api key: ");
    stdin().read_line(&mut api_key).unwrap();

    println!("domain: ");
    stdin().read_line(&mut domain).unwrap();

    println!("user: ");
    stdin().read_line(&mut user).unwrap();

    println!("password length: ");
    stdin().read_line(&mut password_length).unwrap();

    let password_length = password_length.trim().parse::<usize>().unwrap();
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(password_length)
        .map(char::from)
        .collect();

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(format!(
            "https://api.mailgun.net/v3/domains/{}/credentials",
            domain.trim()
        ))
        .basic_auth("api", Some(api_key.trim()))
        .form(&[("login", user.trim()), ("password", password.as_str())])
        .send()?;

    if res.status() == 200 {
        println!(
            "user: {}\npassword: {}",
            user.trim().to_string() + "@smtp.omnis1.com",
            password
        );
    } else {
        println!("something went wrong: {}", res.text()?);
    }

    Ok(())
}
