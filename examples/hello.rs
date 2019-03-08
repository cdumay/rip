use std::str::FromStr;

use rip::openapi::v2::Spec as SpecV2;
use rip::openapi::v3::Spec as SpecV3;
use rip::rest::Application;
use rip::rest::application::ApplicationBuilder;
use rip::rest::contact::Contact;
use rip::rest::contact::ContactBuilder;
use rip::rest::documentation::Documentation;
use rip::rest::documentation::DocumentationBuilder;
use rip::rest::license::{License, OpenSourceLicenses};
use rip::rest::scheme::Scheme;
use rip::rest::server::Server;
use rip::rest::server::ServerBuilder;

fn main() {
    let app = Application::new(env!("CARGO_PKG_NAME").to_string(), env!("CARGO_PKG_VERSION").to_string())
        .set_license(License::from(OpenSourceLicenses::MIT))
        .set_base_url(format!("/api/{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR")))
        .set_scheme(vec![Scheme::Http])
        .set_consumes(vec![mime::APPLICATION_JSON])
        .set_produces(vec![mime::APPLICATION_JSON])
        .set_description(env!("CARGO_PKG_DESCRIPTION").to_string())
        .set_tos(url::Url::from_str("https://en.wikipedia.org/wiki/Terms_of_service").unwrap())
        .set_contact(
            Contact::new("Cedric Dumay".to_string())
                .set_email("cedric.dumay@gmail.com".to_string())
                .set_url(url::Url::from_str("https://github.com/cdumay").unwrap())
        )
        .set_servers(vec![
            Server::new(url::Url::from_str("http://example.com").unwrap())
        ])
        .set_documentation(
            Documentation::new(url::Url::from_str("https://github.com/cdumay/rip").unwrap())
                .set_description("Source code".to_string())
        )
        ;
    println!("OpenApi V2");
    println!("{}", serde_json::to_string_pretty(&SpecV2::from(&app)).unwrap());
    println!("OpenApi V3");
    println!("{}", serde_json::to_string_pretty(&SpecV3::from(&app)).unwrap());
}