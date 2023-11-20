use crate::counting::{Attr, SwEvent};
use crate::{Builder, EventScope};

fn workload() {
    for _ in 0..10000000 {
        std::hint::black_box(0);
    }
}

#[test]
fn test() {
    let builder = Builder::new().calling_process().any_cpu();
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        Attr::new(event, scopes, Default::default())
    };
    let mut counting = builder.build_counting(attr).unwrap();

    let before = counting.get_result().unwrap().event_count;
    dbg!(before);
    assert_eq!(before, 0);
    counting.enable().unwrap();

    workload();

    counting.disable().unwrap();
    let after = counting.get_result().unwrap().event_count;
    dbg!(after);
    assert!(after > 0);
    assert_eq!(after, counting.get_result().unwrap().event_count);

    counting.enable().unwrap();
    assert_ne!(after, counting.get_result().unwrap().event_count);
}
