use rayon::ThreadPool;

pub fn build_thread_pool() -> Result<ThreadPool, anyhow::Error> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()?;
    Ok(pool)
}