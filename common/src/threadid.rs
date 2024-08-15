use log::info;
use rand::{thread_rng, Rng};
use std::cell::Cell;

thread_local! {
    static TID: Cell<Option<u32>> = Cell::new(None);
}

pub fn gettid() -> u32 {
    let tid = TID.get();
    if let Some(ret) = tid {
        return ret;
    }

    let newid = thread_rng().gen::<u32>();
    info!("New thread ID: {}", newid);
    TID.set(Some(newid));
    newid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test() {
        let tid1a = gettid();
        let tid1b = gettid();

        std::thread::spawn(move || {
            let tid2a = gettid();
            let tid2b = gettid();

            std::thread::spawn(move || {
                let tid3a = gettid();
                let tid3b = gettid();

                assert_eq!(tid1a, tid1b);
                assert_eq!(tid2a, tid2b);
                assert_eq!(tid3a, tid3b);

                assert_ne!(tid1a, tid2a);
                assert_ne!(tid2a, tid3a);
                assert_ne!(tid3a, tid1a);
            });
        });
    }
}
