use std::cmp::min;
use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner:.blue} {msg} {percent}% ({eta})").unwrap());
    pb.set_message("Downloading asset");
    pb.enable_steady_tick(Duration::from_millis(80));

    // Simulate download total size from network call...
    thread::sleep(Duration::from_millis(500));

    let total_size = 231231231;
    pb.set_length(total_size);

    let mut downloaded = 0;
    while downloaded < total_size {
        let new = min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(6));
    }

    pb.finish_and_clear();
    println!("downloaded asset");
}
