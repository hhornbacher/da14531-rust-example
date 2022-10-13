use da14531_sdk::{
    app_modules::app_custs::{custs1::app_custs1_create_db, CustPrfFuncCallbacks},
    app_modules::{
        default_handlers_configuration, ms_to_timer_units, DEF_ADV_WITH_TIMEOUT, DEF_SEC_REQ_NEVER,
    },
    ble_stack::profiles::custom::custs::service_database,
    perm,
    platform::core_modules::rwip::TASK_ID_CUSTS1,
};

// Setup service database
service_database![
    {
        etype: service,
        uuid16: 0xFD6B // Rapitag 16bit UUID
    },
    {
        etype: characteristic,
        perm: perm!(WR, ENABLE) | perm!(WRITE_COMMAND, ENABLE) | perm!(WRITE_REQ, ENABLE),
        uuid16: 0x0002,
        length: 1, // bool
        user_description: "LED Write"
    },
    {
        etype: characteristic,
        perm: perm!(RD, ENABLE),
        uuid16: 0x0003,
        length: 1, // bool
        user_description: "LED Read"
    },
    {
        etype: characteristic,
        perm: perm!(RD, ENABLE),
        uuid16: 0x0004,
        length: 2, // u16
        user_description: "Temperature Read"
    }
];

/// Setup custom profile funcs
#[no_mangle]
pub static CUST_PRF_FUNCS: [CustPrfFuncCallbacks; 1] = [CustPrfFuncCallbacks {
    task_id: TASK_ID_CUSTS1,
    att_db: &CUSTS1_ATT_DB as *const _ as *const da14531_sdk::bindings::attm_desc_128,
    max_nb_att: CUSTS1_ATT_DB_LEN,
    db_create_func: Some(app_custs1_create_db),
    enable_func: None,
    init_func: None,
    value_wr_validation_func: None,
}];

/// Set the advertisement period
const ADV_PERIOD: i32 = ms_to_timer_units(4000) as i32;

// Configure default handlers
default_handlers_configuration! {
    adv_scenario: DEF_ADV_WITH_TIMEOUT,
    advertise_period: ADV_PERIOD,
    security_request_scenario: DEF_SEC_REQ_NEVER
}
