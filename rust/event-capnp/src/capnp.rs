use std::sync::Mutex;

use crate::event::Event;
use capnp_util::CapnpReusableWriter;
use once_cell::sync::Lazy;

static CAPNP_WRITER: Lazy<Mutex<CapnpReusableWriter>> =
    Lazy::new(|| Mutex::new(CapnpReusableWriter::new()));

fn mutex_error_to_capnp_error<T>(e: std::sync::PoisonError<T>) -> capnp::Error {
    capnp::Error::failed(e.to_string())
}

mod event_capnp {
    include!(concat!(env!("OUT_DIR"), "/event_capnp.rs"));
}

pub fn create_event(event: Event) -> capnp::Result<Vec<u8>> {
    let mut writer = CAPNP_WRITER.lock().map_err(mutex_error_to_capnp_error)?;
    let mut builder = writer.builder();
    let mut root = builder.init_root::<event_capnp::event::Builder>();

    root.set_name(event.name.as_str());
    root.set_year(event.year);

    let mut data = Vec::new();
    capnp::serialize::write_message(&mut data, &builder)?;

    Ok(data)
}

pub fn read_event(event: &[u8]) -> capnp::Result<Event> {
    let mut data_slice: &[u8] = event.as_ref();
    let message =
        capnp::serialize::read_message_from_flat_slice(&mut data_slice, Default::default())?;

    let root = message.get_root::<event_capnp::event::Reader>()?;

    let event = Event {
        name: root.get_name()?.to_string()?,
        year: root.get_year(),
    };

    Ok(event)
}

pub fn modify_event(event: &mut [u8]) -> capnp::Result<()> {
    let mut data_slice: &[u8] = event.as_ref();

    // Reads a serialized message (including a segment table) from a flat slice of bytes, without copying.
    // The slice is allowed to extend beyond the end of the message. On success, updates slice to point to the remaining bytes beyond the end of the message.
    let message =
        capnp::serialize::read_message_from_flat_slice(&mut data_slice, Default::default())?;

    let root = message.get_root::<event_capnp::event::Reader>()?;

    let mut writer = CAPNP_WRITER.lock().map_err(mutex_error_to_capnp_error)?;
    let mut builder = writer.builder();
    builder.set_root(root)?;

    let mut root = builder.get_root::<event_capnp::event::Builder>()?;
    root.set_year(2025); // or any other edit

    let mut cursor = std::io::Cursor::new(event);
    let mut cursor = cursor.get_mut();
    capnp::serialize::write_message(&mut cursor, &builder)
}
