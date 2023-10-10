use capnp::message::ScratchSpaceHeapAllocator;

pub struct CapnpReusableWriter<'a> {
    _words: Vec<capnp::Word>,
    allocator: ScratchSpaceHeapAllocator<'a>,
}

impl<'a> CapnpReusableWriter<'a> {
    pub fn new() -> CapnpReusableWriter<'a> {
        let mut words = capnp::Word::allocate_zeroed_vec(188);
        let scratch_space: &mut [u8] =
            capnp::Word::words_to_bytes_mut(unsafe { std::mem::transmute(&mut words[..]) });
        let allocator = ScratchSpaceHeapAllocator::new(scratch_space);

        Self {
            _words: words,
            allocator,
        }
    }

    pub fn builder(&mut self) -> capnp::message::Builder<&mut ScratchSpaceHeapAllocator<'a>> {
        capnp::message::Builder::new(&mut self.allocator)
    }
}
