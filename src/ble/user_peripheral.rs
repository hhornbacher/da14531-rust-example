use da14531_sdk::{
    app_modules::app_env_get_conidx,
    ble_stack::{
        host::gap::gapc::task::GAPC_PARAM_UPDATED_IND,
        profiles::custom::custs::custs1::task::{
            Custs1AttInfoReq, Custs1ValWriteInd, Custs1ValueReqInd, KeMsgCusts1AttInfoRsp,
            KeMsgCusts1ValueReqRsp, CUSTS1_ATT_INFO_REQ, CUSTS1_VALUE_REQ_IND,
            CUSTS1_VAL_WRITE_IND,
        },
        rwble_hl::error::HlErr::{ATT_ERR_APP_ERROR, ATT_ERR_WRITE_NOT_PERMITTED},
    },
    platform::core_modules::ke::{msg::KeMsgId, task::KeTaskId},
};

use super::char_handlers::{
    led_read_char_read_handler, led_write_char_write_handler, temp_read_char_read_handler,
};

// This whole thing needs to be simplified with macros!!
// These are the indices of the entries in the service database
const SVC1_IDX_UNLOCK_VAL: u16 = 2;
const SVC1_IDX_LED_READ_VAL: u16 = 5;
const SVC1_IDX_LED_WRITE_VAL: u16 = 8;
const SVC1_IDX_TEMP_READ_VAL: u16 = 11;

#[no_mangle]
pub fn user_catch_rest_hndl(
    msg_id: KeMsgId,
    param: *const cty::c_void,
    dest_id: KeTaskId,
    src_id: KeTaskId,
) {
    // rprintln!(
    //     "user_catch_rest_hndl({}, {:p}, {}, {})",
    //     msg_id,
    //     param,
    //     dest_id,
    //     src_id
    // );
    if msg_id == GAPC_PARAM_UPDATED_IND as u16 {
        // let param = param as *const GapcParamUpdatedInd;
        // let param = unsafe { &*param };
        // let con_interval = param.con_interval;
        // let con_latency = param.con_latency;
        // let sup_to = param.sup_to;

        // // Check if updated Conn Params filled to preferred ones
        // if (con_interval >= USER_CONNECTION_PARAM_CONF.intv_min)
        //     && (con_interval <= USER_CONNECTION_PARAM_CONF.intv_max)
        //     && (con_latency == USER_CONNECTION_PARAM_CONF.latency)
        //     && (sup_to == USER_CONNECTION_PARAM_CONF.time_out)
        // {}

        return;
    }

    match msg_id as u32 {
        CUSTS1_VAL_WRITE_IND => {
            let param = param as *const Custs1ValWriteInd;
            let param = unsafe { &*param };
            match param.handle {
                SVC1_IDX_LED_WRITE_VAL => {
                    led_write_char_write_handler(param);
                }
                _ => {}
            }
        }
        CUSTS1_ATT_INFO_REQ => {
            let param = param as *const Custs1AttInfoReq;
            let param = unsafe { *param };
            let att_idx = param.att_idx;
            match att_idx {
                _ => {
                    let mut response = KeMsgCusts1AttInfoRsp::new(src_id, dest_id);

                    let conidx = app_env_get_conidx(param.conidx);

                    // Provide the connection index.
                    response.fields().conidx = conidx;

                    // Provide the attribute index.
                    response.fields().att_idx = param.att_idx;

                    // Force current length to zero.
                    response.fields().length = 0;

                    // Provide the ATT error code.
                    response.fields().status = ATT_ERR_WRITE_NOT_PERMITTED as u8;

                    response.send();
                }
            }
        }
        CUSTS1_VALUE_REQ_IND => {
            let param = param as *const Custs1ValueReqInd;
            let param = unsafe { &*param };
            let att_idx = param.att_idx;

            match att_idx {
                SVC1_IDX_LED_READ_VAL => led_read_char_read_handler(param),
                SVC1_IDX_TEMP_READ_VAL => temp_read_char_read_handler(param),
                _ => {
                    let mut response = KeMsgCusts1ValueReqRsp::new(dest_id, src_id);

                    // Provide the connection index.
                    response.fields().conidx = app_env_get_conidx(param.conidx);

                    // Provide the attribute index.
                    response.fields().att_idx = param.att_idx;

                    // Force current length to zero.
                    response.fields().length = 0;

                    // Provide the ATT error code.
                    response.fields().status = ATT_ERR_APP_ERROR as u8;

                    response.send();
                }
            }
        }
        _ => {}
    }
}
