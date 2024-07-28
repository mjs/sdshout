use std::time::Duration;

use dbus::arg;
use dbus::arg::messageitem::MessageItemArray;
use dbus::blocking::Connection;
use dbus::Message;

// TODO
// - split out watching in a module, make it take a notify fn
// - report dead services on startup
// - rate limiting

/// Generated using:
/// dbus-codegen-rust -s -p /org/freedesktop/systemd1 -d org.freedesktop.systemd1
///

pub struct OrgFreedesktopSystemd1ManagerJobRemoved {
    pub id: u32,
    pub job: dbus::Path<'static>,
    pub unit: String,
    pub result: String,
}

impl arg::ReadAll for OrgFreedesktopSystemd1ManagerJobRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopSystemd1ManagerJobRemoved {
            id: i.read()?,
            job: i.read()?,
            unit: i.read()?,
            result: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopSystemd1ManagerJobRemoved {
    const NAME: &'static str = "JobRemoved";
    const INTERFACE: &'static str = "org.freedesktop.systemd1.Manager";
}

// END Generated

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

    systemd_proxy.match_signal(handle_job_removed).unwrap(); // XXX avoid

    // Loop and print out all messages received (using handle_message()) as they come.
    // Some can be quite large, e.g. if they contain embedded images..
    loop {
        conn.process(Duration::from_millis(1000)).unwrap();
    }
}

fn handle_job_removed(
    h: OrgFreedesktopSystemd1ManagerJobRemoved,
    _: &Connection,
    _: &Message,
) -> bool {
    notify(&h.unit, &h.result);
    true
}

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

fn main() {
    watch();
}
