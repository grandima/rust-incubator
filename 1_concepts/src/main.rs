mod safe_list;
use rand::Rng;
use safe_list::SafeList;
use std::{thread::*, time::Duration, sync::{Mutex, Arc}};
fn main() {
    let list: SafeList<i32> = SafeList::new();
    let list2 = list.clone();
    let should_stop: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let should_stop2 = should_stop.clone();
    spawn({
        move || {
            let mut rng = rand::thread_rng();
            loop {
                let val = rng.gen_range(0..100);
                println!("push: {:?}", val);
                if val == 0 {
                    *should_stop.lock().unwrap() = true;
                    break
                }
                if val % 2 == 0 {
                    list.push_front(val);
                } else {
                    list.push_back(val);
                }
                sleep(Duration::from_millis(100));
            }
        }
    })
    .join()
    .unwrap();
    spawn(move || {
        loop {
            let val = list2.pop_back();
            if val == Some(0) {
                break;
            } else if val == None {
                if *should_stop2.lock().unwrap() {
                    break
                }
                sleep(Duration::from_millis(10));  
            }
            sleep(Duration::from_millis(100));
        }
    })
    .join()
    .unwrap();
}
