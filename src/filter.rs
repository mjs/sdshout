use crate::notify;

pub fn filter(unit_name: &str, result: &str) {
    notify::notify(notify::NotifyInfo {
        message: format!("Unit {} has {}", unit_name, result), // XXX Less awkward message
        timeout: 5000,                                         // XXX from config
    });
}
