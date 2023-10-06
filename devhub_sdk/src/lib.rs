pub use hdk_extensions::hdi;
pub use hdk_extensions::holo_hash;
pub use hdk_extensions::hdk;
pub use hdk_extensions::hdi_extensions;
pub use hdk_extensions;

use hdk::prelude::*;

/// Get a microsecond timestamp of now
pub fn timestamp() -> ExternResult<u64> {
    Ok( sys_time().map( |t| (t.as_micros() / 1000) as u64 )? )
}
