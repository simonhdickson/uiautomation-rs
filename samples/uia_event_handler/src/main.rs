use std::{thread, time::Duration};

use uiautomation::{handlers::EventId, types::TreeScope, UIAutomation};

fn main() {
    let automation = UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();
    let cache_request = automation.create_cache_request().unwrap();

    automation
        .add_automation_event_handler(
            EventId::InvokeInvokedEventId,
            &root,
            TreeScope::Descendants,
            &cache_request,
            |element| {
                println!("clicked element: {:?}", element);
            },
        )
        .unwrap();

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
