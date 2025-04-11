use crate::certificates::crt::Certificates;
use crate::ports::port::Port;
use rayon::prelude::*;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashSet;
use std::time::Duration;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::proto::rr::RecordType;
use trust_dns_resolver::Resolver;

#[derive(Debug, Deserialize, Clone)]
pub struct Subdomains {
    pub subdomain: String,
    pub open_ports: Vec<Port>,
}

impl Subdomains {
    pub fn create(client: &Client, domain: &str) -> Result<Vec<Subdomains>, anyhow::Error> {
        let url = format!("https://crt.sh/?q=%25.{}&output=json", domain);
        let crts: Vec<Certificates> = client.get(&url).send()?.json()?;
        let mut filtered_domains = filter_domains(crts, domain);
        filtered_domains.insert(domain.into());
        let subdomains = add_port(filtered_domains);
        Ok(subdomains)
    }
}
fn filter_domains(crts: Vec<Certificates>, domain: &str) -> HashSet<String> {
    crts.into_par_iter()
        .flat_map(|crt| {
            crt.name_value
                .split("\n")
                .map(|subdomains| subdomains.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain| subdomain != domain)
        .filter(|subdomain| !subdomain.contains('*'))
        .collect()
}

fn add_port(subdomains: HashSet<String>) -> Vec<Subdomains> {
    subdomains
        .into_iter()
        .map(|subdomain| Subdomains {
            subdomain,
            open_ports: Vec::new(),
        })
        .filter(resolves)
        .collect()
}

fn resolves(subdomain: &Subdomains) -> bool {
    let mut resolver_opts = ResolverOpts::default();
    let resolver_config = ResolverConfig::default();

    resolver_opts.timeout = Duration::from_secs(4);
    let dns_resolve = Resolver::new(resolver_config, resolver_opts).unwrap();
    dns_resolve.lookup(&subdomain.subdomain, RecordType::A).is_ok()
}
