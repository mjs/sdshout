use std::time::Duration;

use dbus::blocking::Connection;
use dbus::channel::MatchingReceiver;
use dbus::message::MatchRule;
use dbus::Message;

// This programs implements the equivalent of running the "dbus-monitor" tool
fn main() {
    // First open up a connection to the desired bus.
    let conn = Connection::new_system().expect("D-Bus connection failed");

    let systemd_proxy = conn.with_proxy(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        Duration::from_millis(5000),
    );

    let result: Result<(), dbus::Error> =
        systemd_proxy.method_call("org.freedesktop.systemd1.Manager", "Subscribe", ());
    match result {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Subscribe failed: '{}'", err);
            return;
        }
    }

    let dbus_proxy = conn.with_proxy(
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        Duration::from_millis(5000),
    );
    // Second create a rule to match messages we want to receive; in this example we add no
    // further requirements, so all messages will match
    let rule = MatchRule::new();

    let result: Result<(), dbus::Error> = dbus_proxy.method_call(
        "org.freedesktop.DBus.Monitoring",
        "BecomeMonitor",
        (vec![rule.match_str()], 0u32),
    );

    match result {
        // BecomeMonitor was successful, start listening for messages
        Ok(_) => {
            conn.start_receive(
                rule,
                Box::new(|msg, _| {
                    handle_message(&msg);
                    true
                }),
            );
        }
        Err(e) => {
            eprintln!("Failed to BecomeMonitor: '{}'", e);
            return;
        }
    }

    // Loop and print out all messages received (using handle_message()) as they come.
    // Some can be quite large, e.g. if they contain embedded images..
    loop {
        conn.process(Duration::from_millis(1000)).unwrap();
    }
}

fn handle_message(msg: &Message) {
    println!("Got message: {:?}", msg);
}
