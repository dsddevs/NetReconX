use crate::clients::client::build_client;
use crate::errors::error::Errors;
use crate::scanners::scanner::print_scan_result;
use crate::threads::thread::build_thread_pool;

mod certificates;
mod clients;
mod errors;
mod ports;
mod subdomains;
mod scanners;
mod threads;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Errors::CliErr.into());
    }

    let domain = &args[1];
    let client = build_client()?;
    let thread_pool = build_thread_pool()?;
    thread_pool.install(|| print_scan_result(&client, domain))?;

    Ok(())
}
