mod watch;

use std::time::Duration;

use color_eyre::eyre::Result;
use dbus::arg::messageitem::MessageItemArray;
use dbus::blocking::Connection;

// TODO
// - github
// - split out watching in a module, make it take a notify fn
// - figure out icons
// - report dead services on startup
// - release
// - rate limiting and/or aggregation
// - configurable delay
// - ignore some units

fn notify(unit_name: &str, result: &str) {
    let conn = Connection::new_session().expect("D-Bus connection failed");

    let proxy = conn.with_proxy(
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        Duration::from_millis(5000),
    );

    let actions = MessageItemArray::new(vec![], "as".into()).unwrap();
    let hints = MessageItemArray::new(vec![], "a{sv}".into()).unwrap();

    let result: Result<(), dbus::Error> = proxy.method_call(
        "org.freedesktop.Notifications",
        "Notify",
        (
            "sdshout",
            0u32,
            "", // app_icon
            format!("Local unit {} is {}", unit_name, result),
            "",
            actions,
            hints,
            15000i32, // expire timeout (ms)
        ),
    );
    match result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("notify failed: '{}'", err);
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    watch::watch_units(notify)
}
