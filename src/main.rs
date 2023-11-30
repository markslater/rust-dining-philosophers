use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let forks = vec![Arc::new(Mutex::new(())), Arc::new(Mutex::new(())), Arc::new(Mutex::new(()))];
    let philosophers: Vec<JoinHandle<()>> = forks
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let forks: Vec<Arc<Mutex<()>>> = forks.iter().map(|fork| Arc::clone(fork)).collect();
            thread::spawn(move || {
                let _left_fork = forks[i].lock();
                let right_fork_index = (i + 1) % forks.len();
                println!("Philosopher {i} got fork {i}; going for fork {right_fork_index} next.");
                let _right_fork = forks[right_fork_index].lock();
                println!("Philosopher {i} got fork {right_fork_index}; eating.");
                println!("Philosopher {i} relinquishing forks {i} and {right_fork_index}")
            })
        })
        .collect();
    for philosopher in philosophers {
        philosopher.join().unwrap();
    }
}
