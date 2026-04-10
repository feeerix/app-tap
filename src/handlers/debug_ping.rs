use crate::AppSW;
use ledger_device_sdk::io::{Command, CommandResponse};

/// Deterministic payload returned by INS=0x11 to validate APDU round-trip.
const DEBUG_PING_RESPONSE: [u8; 8] = [0x50, 0x49, 0x4E, 0x47, 0x90, 0xAB, 0xCD, 0xEF];

pub fn handler_debug_ping(command: Command<'_>) -> Result<CommandResponse<'_>, AppSW> {
    // Keep this command strict and tiny: no payload accepted, fixed payload returned.
    if !command.get_data().is_empty() {
        return Err(AppSW::WrongApduLength);
    }
    let mut response = command.into_response();
    response.append(&DEBUG_PING_RESPONSE)?;
    Ok(response)
}
