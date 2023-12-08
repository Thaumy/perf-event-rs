use crate::sampling::record::SampleId;
use crate::syscall::bindings::BPF_TAG_SIZE;

mod raw;

#[derive(Debug, Clone)]
pub struct Body {
    pub r#type: u16,
    pub flags: u16,
    pub id: u32,
    pub tag: [u8; BPF_TAG_SIZE as usize],
    pub sample_id: Option<SampleId>,
}

type RawBody = raw::Body;

impl Body {
    pub unsafe fn from_ptr(ptr: *const u8, sample_id_all: bool) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();

        Self {
            r#type: raw.r#type,
            flags: raw.flags,
            id: raw.id,
            tag: raw.tag,
            sample_id: sample_id_all.then(|| raw.sample_id().clone()),
        }
    }
}
