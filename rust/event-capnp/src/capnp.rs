use crate::event::Event;
use capnp_util::CapnpReusableWriter;
use once_cell::sync::Lazy;

static mut CAPNP_WRITER: Lazy<CapnpReusableWriter> = Lazy::new(CapnpReusableWriter::new);

mod event_capnp {
    include!(concat!(env!("OUT_DIR"), "/event_capnp.rs"));
}

pub fn create_event(event: Event) -> capnp::Result<Vec<u8>> {
    let mut builder = unsafe { CAPNP_WRITER.builder() };
    let mut root = builder.init_root::<event_capnp::event::Builder>();

    root.set_name(event.name.as_str().into());
    root.set_year(event.year);

    let mut data = Vec::new();
    capnp::serialize::write_message(&mut data, &builder)?;

    Ok(data)
}

pub fn read_event(data: &[u8]) -> capnp::Result<Event> {
    let mut data_slice: &[u8] = data.as_ref();
    let message =
        capnp::serialize::read_message_from_flat_slice(&mut data_slice, Default::default())?;

    let root = message.get_root::<event_capnp::event::Reader>()?;

    let event = Event {
        name: root.get_name()?.to_string()?,
        year: root.get_year(),
    };

    Ok(event)
}

pub fn modify_event(data: &mut [u8]) -> capnp::Result<()> {
    let mut data_slice: &[u8] = data.as_ref();
    let message =
        capnp::serialize::read_message_from_flat_slice(&mut data_slice, Default::default())?;

    let root = message.get_root::<event_capnp::event::Reader>()?;

    let mut builder = unsafe { CAPNP_WRITER.builder() };
    builder.set_root(root)?;

    let mut root = builder.get_root::<event_capnp::event::Builder>()?;
    root.set_year(2024); // or any other edit

    let mut cursor = std::io::Cursor::new(data);
    let mut cursor = cursor.get_mut();
    capnp::serialize::write_message(&mut cursor, &builder)
}
