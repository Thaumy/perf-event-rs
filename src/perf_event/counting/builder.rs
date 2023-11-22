use crate::counting::{Attr, Counting, CountingGroup};
use crate::infra::result::WrapResult;
use crate::{BuildError, Builder};

impl Builder {
    // TODO
    pub fn build_counting(&self, attr: Attr) -> Result<Counting, BuildError> {
        match self {
            Builder {
                pid: None,
                cpu: None,
                ..
            } => Err(BuildError::PidAndCpuNotSet),
            Builder {
                pid: Some(pid),
                cpu: Some(cpu),
                ..
            } => {
                unsafe { Counting::new(attr, *pid, *cpu, -1, 0) }.map_err(BuildError::SyscallFailed)
            }
            _ => todo!(),
        }
    }

    pub fn build_counting_group(&self) -> Result<CountingGroup, BuildError> {
        match self {
            Builder {
                pid: None,
                cpu: None,
                ..
            } => Err(BuildError::PidAndCpuNotSet),
            Builder {
                pid: Some(pid),
                cpu: Some(cpu),
                ..
            } => unsafe { CountingGroup::new(*pid, *cpu) }.wrap_ok(),
            _ => todo!(),
        }
    }
}