/*
struct {
  u16 type;
  u16 flags;
  u32 id;
  u8 tag[BPF_TAG_SIZE];
  struct sample_id sample_id;
};
*/

use crate::sampling::record::SampleId;
use crate::syscall::bindings::BPF_TAG_SIZE;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Body {
    pub r#type: u16,
    pub flags: u16,
    pub id: u32,
    pub tag: [u8; BPF_TAG_SIZE as usize],
}

impl Body {
    pub unsafe fn sample_id(&self) -> &SampleId {
        let ptr = (self as *const Self).add(1) as *const SampleId;
        ptr.as_ref().unwrap()
    }
}