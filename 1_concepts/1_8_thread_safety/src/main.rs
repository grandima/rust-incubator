#![feature(negative_impls)]
use std::{sync::atomic::AtomicU64, os::unix::thread};
use std::thread::spawn;
struct OnlySend{
    val: i32,
}
unsafe impl Send for OnlySend{}
impl !Sync for OnlySend{}

struct OnlySync{
    val: AtomicU64,
}
unsafe impl Sync for OnlySync{}
impl !Send for OnlySync{}

struct SyncAndSend {
    val: AtomicU64,
}

struct NotSyncNotSend {
    val: AtomicU64,
}

impl !Sync for NotSyncNotSend{}
impl !Send for NotSyncNotSend{}
/* In order to create a variable to demonstrate that it may 
or may not be shared between threads, it needs to have 
the lifetime that is equal or longer than each thread's it is used in.
That's why it needs to be `static`. 
But it cannot be declared as static, because it's !Sync.
 */
//static STATIC_ONLY_SEND: OnlySend = OnlySend{val: 1};
static STATIC_ONLY_SYNC: OnlySync = OnlySync{val: AtomicU64::new(1)};
static STATIC_S_S: SyncAndSend = SyncAndSend{val: AtomicU64::new(1)};
fn main() {
    let send = OnlySend{val: 1};
    let ref_send = &send;
    let sync = OnlySync{val: AtomicU64::new(1)};
    let ref_static_sync = &STATIC_ONLY_SYNC;
    let s_s = SyncAndSend{val: AtomicU64::new(1)};
    let ref_static_s_s = &STATIC_S_S;
    let ns_ns = NotSyncNotSend{val: AtomicU64::new(1)};
    
    spawn(||{
        _ = &STATIC_ONLY_SYNC;
        _ = &STATIC_S_S;
    }).join().unwrap();
    spawn( move || {
        //cannot borrow because Not Sync. compile error
        // ref_send;
        send;
        ref_static_sync;
        //cannot move because Not Send. compile error
        // sync;
        s_s;
        ref_static_s_s;
        //cannot move because Not Send. compile error
        // ns_ns;
    }).join().unwrap();

    spawn({
        let ref_ns_ns = &ns_ns;
        move ||{
            //cannot borrow Not Sync. compile error
            // ref_ns_ns;
        }
    }).join().unwrap();

}
