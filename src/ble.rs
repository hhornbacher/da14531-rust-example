use da14531_sdk::app_modules::{
    app_common::app::{app_easy_gap_advertise_stop, app_easy_gap_undirected_advertise_start},
    app_easy_gap_disconnect,
};

use crate::app::BleDriver;

pub mod char_handlers;
pub mod config;
pub mod user_peripheral;

pub struct Da14531Ble;

impl BleDriver for Da14531Ble {
    fn start_adverstising() {
        app_easy_gap_undirected_advertise_start();
    }

    fn stop_adverstising() {
        app_easy_gap_advertise_stop();
    }

    fn disconnect(connection_handle: u32) {
        app_easy_gap_disconnect(connection_handle as u8);
    }
}
