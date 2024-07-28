use std::time::Duration;

use dbus::arg::messageitem::MessageItemArray;
use dbus::arg::TypeMismatchError;
use dbus::blocking::Connection;
use dbus::channel::MatchingReceiver;
use dbus::message::MatchRule;
use dbus::strings::Path;
use dbus::Message;

fn watch() {
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

    let rule = MatchRule::new_signal("org.freedesktop.systemd1.Manager", "JobRemoved");

    // XXX probabyl not needed
    // see signal receiving here: https://github.com/diwic/dbus-rs/blob/master/dbus/examples/match_signal.rs
    let result: Result<(), dbus::Error> = dbus_proxy.method_call(
        "org.freedesktop.DBus.Monitoring",
        "BecomeMonitor",
        (vec![rule.match_str()], 0u32),
    );

    match result {
        Ok(_) => {
            conn.start_receive(
                rule,
                Box::new(|msg, _| {
                    handle_job_removed(&msg);
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

fn handle_job_removed(msg: &Message) {
    let result: Result<(u32, Path, &str, &str), TypeMismatchError> = msg.read4();
    match result {
        Ok((_, _, unit, unit_result)) => {
            println!("{} stopped with {}", unit, unit_result);
        }
        Err(e) => {
            eprintln!("reading message failed: {:?}", e);
        }
    }
}

fn notify() {
    let conn = Connection::new_session().expect("D-Bus connection failed");

    let proxy = conn.with_proxy(
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        Duration::from_millis(5000),
    );

    let x = MessageItemArray::new(vec![], "as".into()).unwrap();
    let y = MessageItemArray::new(vec![], "a{sv}".into()).unwrap();

    let result: Result<(), dbus::Error> = proxy.method_call(
        "org.freedesktop.Notifications",
        "Notify",
        ("sdshout", 123u32, "", "foo", "body boyy", x, y, 5000i32),
    );
    match result {
        Ok(_) => println!("succces"),
        Err(err) => {
            eprintln!("Subscribe failed: '{}'", err);
        }
    }
}

fn main() {
    // XXX maybe becomemonitor is unnecessary??
    watch();
}
