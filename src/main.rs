extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::Deserialize;

// struct para armazenar os dados de resposta da API SecurityTrails
#[derive(Deserialize)]
struct SecurityTrailsResponse {
    subdomains: Vec<String>,
}

// struct para armazenar os dados de resposta da API Shodan
#[derive(Deserialize)]
struct ShodanResponse {
    data: Vec<ShodanData>,
}

#[derive(Deserialize)]
struct ShodanData {
    ip_str: String,
    hostnames: Option<Vec<String>>,
}

fn main() {
    let api_key = "SUA_CHAVE_DA_API";
    let domain = "example.com";

    // fazendo a chamada da API SecurityTrails
    let security_trails_url = format!("https://api.securitytrails.com/v1/domain/{}/subdomains", domain);
    let security_trails_response = reqwest::Client::new()
        .get(&security_trails_url)
        .header("apikey", api_key)
        .send()
        .unwrap()
        .json::<SecurityTrailsResponse>()
        .unwrap();

    println!("Subdomínios encontrados usando SecurityTrails:");
    for subdomain in security_trails_response.subdomains {
        println!("- {}", subdomain);
    }

    // fazendo a chamada da API Shodan
    let shodan_url = format!("https://api.shodan.io/dns/domain/{}", domain);
    let shodan_response = reqwest::Client::new()
        .get(&shodan_url)
        .header("apikey", api_key)
        .send()
        .unwrap()
        .json::<ShodanResponse>()
        .unwrap();

    println!("\nSubdomínios encontrados usando Shodan:");
    for data in shodan_response.data {
        match data.hostnames {
            Some(hostnames) => {
                for hostname in hostnames {
                    println!("- {}", hostname);
                }
            }
            None => (),
        }
    }
}



