use std::time::Duration;

use dbus::arg;
use dbus::arg::messageitem::MessageItemArray;
use dbus::arg::TypeMismatchError;
use dbus::blocking::Connection;
use dbus::strings::Path;
use dbus::Message;

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

    systemd_proxy
        .match_signal(
            |h: OrgFreedesktopSystemd1ManagerJobRemoved, _: &Connection, _: &Message| {
                println!("JobRemoved: {} {} {}", h.job, h.unit, h.result);
                true
            },
        )
        .unwrap();

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
    watch();
}
