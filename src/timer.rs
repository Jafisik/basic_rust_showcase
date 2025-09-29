use tokio::sync::mpsc;
use tokio::time::{sleep, Duration, Instant};

pub enum Command {
    Stop,
}

pub async fn timer(mut rx: mpsc::Receiver<Command>, duration: u64) {
    let total = Duration::from_secs(duration);
    let mut elapsed = Duration::from_secs(0);
    let mut last_tick = Instant::now();

    loop {
        tokio::select! {
            Some(cmd) = rx.recv() => {
                match cmd {
                    Command::Stop => { 
                        let total_num: i32 = total.as_secs() as i32;
                        let elapsed_num: i32 = elapsed.as_secs() as i32;
                        println!("Zastaveno. Čas od nuly: {}", total_num-elapsed_num);
                        break;
                    }
                }
            }
            _ = sleep(Duration::from_secs(1)) => {
                let now = Instant::now();
                elapsed += now - last_tick;
                last_tick = now;
                
                if elapsed.as_secs() <= 3{
                    println!("Zbývá: {}s", total.as_secs() as i32 - elapsed.as_secs() as i32);
                }
            }
        }
    }
}