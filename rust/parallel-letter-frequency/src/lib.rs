use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input: Arc<Vec<_>> = Arc::new(input.iter().flat_map(|c| c.chars()).collect());
    let slice_len = input.len() / worker_count;
    let mut workers = Vec::with_capacity(worker_count);

    for i in 0..worker_count {
        let my_input = Arc::clone(&input);
        let handle = thread::spawn(move || {
            let start = slice_len * i;
            let end = if i == worker_count - 1 {
                my_input.len()
            } else {
                start + slice_len
            };
            let mut map = HashMap::with_capacity(26);
            for c in my_input[start..end]
                .iter()
                .filter(|c| c.is_alphabetic())
                .flat_map(|c| c.to_lowercase())
            {
                *map.entry(c).or_insert(0) += 1;
            }
            map
        });
        workers.push(handle);
    }

    let mut freq = HashMap::with_capacity(26);
    for worker in workers {
        if let Ok(map) = worker.join() {
            for (k, v) in map {
                *freq.entry(k).or_insert(0) += v;
            }
        }
    }
    freq
}
