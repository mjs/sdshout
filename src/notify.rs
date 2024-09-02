use std::collections::HashMap;
use std::time::Duration;

use dbus::arg::Variant;
use dbus::blocking::Connection;

const CONN_TIMEOUT: Duration = Duration::from_millis(5000);

pub struct NotifyInfo {
    pub message: String,
    pub timeout: i32,
}

pub fn notify(info: NotifyInfo) {
    let conn = Connection::new_session().expect("D-Bus connection failed");

    let proxy = conn.with_proxy(
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        CONN_TIMEOUT,
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
            info.message,
            "",
            actions,
            hints,
            info.timeout,
        ),
    );
    match result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("notify failed: '{}'", err);
        }
    }
}
