use da14531_sdk::{
    app_modules::{
        app_cfg_addr_src, configure_custom_server1_service, default_handlers_configuration,
        ms_to_ble_slots, ms_to_timer_units, AdvertiseConfiguration, APP_CFG_ADDR_STATIC,
        DEF_ADV_WITH_TIMEOUT, DEF_SEC_REQ_NEVER,
    },
    ble_stack::host::gap::GAP_GEN_DISCOVERABLE,
    platform::core_modules::common::{ADV_ALLOW_SCAN_ANY_CON_ANY, ADV_ALL_CHNLS_EN},
};

#[no_mangle]
pub static USER_DEVICE_NAME: &str = "ble-example";

pub const PAYLOAD_LENGTH: u16 = (1024 * 4) + 512;

configure_custom_server1_service! {
    svc1: {
        uuid: 0xBEEF,
        characteristics: {
            led_write: {
                uuid: 0x0001,
                permissions: (WRITE_ENABLED | WRITE_REQUEST_ACCEPTED | WRITE_COMMAND_ACCEPTED),
                length: crate::ble::config::PAYLOAD_LENGTH,
                user_description: "LED Write",
                write_handler: crate::ble::char_handlers::led_write_char_write_handler
            },
            led_read: {
                uuid: 0x0002,
                permissions: (READ_ENABLED),
                length: crate::ble::config::PAYLOAD_LENGTH,
                user_description: "LED Read",
                read_handler: crate::ble::char_handlers::led_read_char_read_handler
            },
            temp_read: {
                uuid: 0x0003,
                permissions: (READ_ENABLED),
                length: crate::ble::config::PAYLOAD_LENGTH,
                user_description: "Temp. Read",
                read_handler: crate::ble::char_handlers::temp_read_char_read_handler
            },
        }
    }
}

/// Set the advertisement period
const ADV_PERIOD: i32 = ms_to_timer_units(4000) as i32;

// Configure default handlers
default_handlers_configuration! {
    adv_scenario: DEF_ADV_WITH_TIMEOUT,
    advertise_period: ADV_PERIOD,
    security_request_scenario: DEF_SEC_REQ_NEVER
}

// Define user-specific advertisement configuration
#[no_mangle]
pub static USER_ADV_CONF: AdvertiseConfiguration = AdvertiseConfiguration {
    addr_src: app_cfg_addr_src(APP_CFG_ADDR_STATIC),
    intv_min: ms_to_ble_slots(100),
    intv_max: ms_to_ble_slots(150),
    channel_map: ADV_ALL_CHNLS_EN as u8,
    mode: GAP_GEN_DISCOVERABLE as u8,
    adv_filt_policy: ADV_ALLOW_SCAN_ANY_CON_ANY as u8,
    peer_addr: [0x1, 0x2, 0x3, 0x4, 0x5, 0x6],
    peer_addr_type: 0,
};
