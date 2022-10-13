use da14531_sdk::{
    app_modules::{app_easy_gap_disconnect, app_env_get_conidx},
    bindings::KE_API_ID_TASK_ID_CUSTS1,
    ble_stack::{
        profiles::{
            custom::custs::custs1::task::{
                Custs1ValWriteInd, Custs1ValueReqInd, KeMsgDynCusts1ValueReqRsp,
            },
            prf::prf_get_task_from_id,
        },
        rwble_hl::error::HlErr::GAP_ERR_NO_ERROR as ATT_ERR_NO_ERROR,
    },
    platform::core_modules::{ke::task::KeTaskId, rwip::TASK_APP},
};

use crate::app_impl::app;

pub fn led_write_char_write_handler(param: &Custs1ValWriteInd) {
    let token = unsafe { param.value.as_slice(1) };

    app().on_set_led(token[0] != 0);

    let conidx = app_env_get_conidx(param.conidx);
    app_easy_gap_disconnect(conidx);
}

pub fn led_read_char_read_handler(param: &Custs1ValueReqInd) {
    let mut response = KeMsgDynCusts1ValueReqRsp::<1>::new(
        TASK_APP as u16,
        prf_get_task_from_id(KE_API_ID_TASK_ID_CUSTS1 as KeTaskId),
    );

    let conidx = app_env_get_conidx(param.conidx);

    // Provide the connection index.
    response.fields().conidx = conidx;

    // Provide the attribute index.
    response.fields().att_idx = param.att_idx;

    // Provide length of the payload (bool = 1)
    response.fields().length = 1;

    // Provide the ATT error code.
    response.fields().status = ATT_ERR_NO_ERROR as u8;

    // Copy value
    let value = unsafe { response.fields().value.as_mut_slice(1) };

    value[0] = if app().get_led_state() { 1 } else { 0 };

    response.send();
}

pub fn temp_read_char_read_handler(param: &Custs1ValueReqInd) {
    let mut response = KeMsgDynCusts1ValueReqRsp::<2>::new(
        TASK_APP as u16,
        prf_get_task_from_id(KE_API_ID_TASK_ID_CUSTS1 as KeTaskId),
    );

    let conidx = app_env_get_conidx(param.conidx);

    // Provide the connection index.
    response.fields().conidx = conidx;

    // Provide the attribute index.
    response.fields().att_idx = param.att_idx;

    // Provide length of the payload (u16 = 2)
    response.fields().length = 2;

    // Provide the ATT error code.
    response.fields().status = ATT_ERR_NO_ERROR as u8;

    // Copy value
    let value = unsafe { response.fields().value.as_mut_slice(2) };

    let temp = app().get_temperature();

    value[0] = ((temp >> 8) & 0xff) as u8;
    value[1] = (temp & 0xff) as u8;

    response.send();
}
