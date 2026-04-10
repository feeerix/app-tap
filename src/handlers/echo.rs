/*****************************************************************************
 *   Ledger Tap App
 *   Felix Voo & Rammy Kim.
 *
 *****************************************************************************/

use crate::AppSW;
use ledger_device_sdk::io::{Command, CommandResponse};

/// Max command data length for ECHO (short APDU Lc fits in one Ledger NFC frame).
const ECHO_MAX_PAYLOAD: usize = 240;

/// Returns command data bytes unchanged (NFC / USB sanity check for the host).
pub fn handler_echo(command: Command<'_>) -> Result<CommandResponse<'_>, AppSW> {
    let (len, buf) = {
        let data = command.get_data();
        if data.len() > ECHO_MAX_PAYLOAD {
            return Err(AppSW::WrongApduLength);
        }
        let len = data.len();
        let mut buf = [0u8; ECHO_MAX_PAYLOAD];
        buf[..len].copy_from_slice(data);
        (len, buf)
    };
    let mut response = command.into_response();
    response.append(&buf[..len])?;
    Ok(response)
}
