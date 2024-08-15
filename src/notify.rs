use std::collections::HashMap;
use std::time::Duration;

use dbus::arg::Variant;
use dbus::blocking::Connection;

pub fn notify(unit_name: &str, result: &str) {
    let conn = Connection::new_session().expect("D-Bus connection failed");

    if result != "failed" {
        return;
    }

    let proxy = conn.with_proxy(
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        Duration::from_millis(5000),
    );

    let actions: Vec<&str> = Vec::new();
    let hints = HashMap::from([("image-path", Variant("dialog-warning"))]);

    let result: Result<(), dbus::Error> = proxy.method_call(
        "org.freedesktop.Notifications",
        "Notify",
        (
            "sdshout",
            0u32,
            "emblem-system", // app_icon
            format!("Unit {} has failed", unit_name),
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
