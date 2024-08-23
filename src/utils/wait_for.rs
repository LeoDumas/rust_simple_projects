pub fn wait_for(num: u64) {
    std::thread::sleep(std::time::Duration::from_secs(num));
}
