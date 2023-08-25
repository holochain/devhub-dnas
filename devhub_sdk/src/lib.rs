pub use hdk_extensions::hdi;
pub use hdk_extensions::holo_hash;
pub use hdk_extensions::hdk;
pub use hdk_extensions::hdi_extensions;
pub use hdk_extensions;

use std::io::Write;

use hdk::prelude::*;
use hdi_extensions::{
    guest_error,
};



/// Get a microsecond timestamp of now
pub fn timestamp() -> ExternResult<u64> {
    Ok( sys_time().map( |t| (t.as_micros() / 1000) as u64 )? )
}


pub fn encode_bundle<T>(bundle: T) -> ExternResult<Vec<u8>>
where
    T: serde::Serialize
{
    let packed_bytes = rmp_serde::to_vec_named( &bundle )
	.map_err( |e| guest_error!(format!("Failed to msgpack bundle: {:?}", e )) )?;
    debug!("Message packed bytes: {}", packed_bytes.len() );

    let mut enc = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    enc.write_all( &packed_bytes )
	.map_err( |e| guest_error!(format!("Failed to gzip package: {:?}", e )) )?;

    let gzipped_package = enc.finish()
	.map_err( |e| guest_error!(format!("Failed to finish gzip encoding: {:?}", e )) )?;
    debug!("Gzipped package bytes: {}", gzipped_package.len() );

    Ok( gzipped_package )
}
