use std::time::Duration;

use dbus::arg;
use dbus::blocking::Connection;
use dbus::Message;
use eyre::{Result, WrapErr};

pub fn watch_units(notify_cb: fn(&str, &str)) -> Result<()> {
    let conn = Connection::new_system().wrap_err("Connection to system D-Bus failed")?;

    let systemd_proxy = conn.with_proxy(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        Duration::from_millis(5000),
    );

    systemd_proxy
        .method_call("org.freedesktop.systemd1.Manager", "Subscribe", ())
        .wrap_err("Subscribe failed")?;

    systemd_proxy
        .match_signal(
            move |h: OrgFreedesktopSystemd1ManagerJobRemoved, _: &Connection, _: &Message| {
                notify_cb(&h.unit, &h.result);
                true
            },
        )
        .wrap_err("match_signal failed")?;

    // Loop and print out all messages received (using handle_message()) as they come.
    // Some can be quite large, e.g. if they contain embedded images..
    loop {
        conn.process(Duration::from_millis(1000)).unwrap();
    }
}

/// Generated using:
/// dbus-codegen-rust -s -p /org/freedesktop/systemd1 -d org.freedesktop.systemd1

pub struct OrgFreedesktopSystemd1ManagerJobRemoved {
    pub _id: u32,
    pub _job: dbus::Path<'static>,
    pub unit: String,
    pub result: String,
}

impl arg::ReadAll for OrgFreedesktopSystemd1ManagerJobRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopSystemd1ManagerJobRemoved {
            _id: i.read()?,
            _job: i.read()?,
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
