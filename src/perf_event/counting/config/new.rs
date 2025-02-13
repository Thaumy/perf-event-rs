// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of perf-event-rs.
//
// Perf-event-rs is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// Perf-event-rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.

use crate::counting::{Config, ExtraConfig};
use crate::perf_event::PerfEventAttr;
use crate::syscall::bindings::*;
#[cfg(feature = "linux-4.17")]
use crate::{DynamicPmuEvent, KprobeConfig, UprobeConfig};
use crate::{Event, EventScope, RawPerfEventAttr};
use std::mem::size_of;

#[inline]
pub fn new<'t>(
    event: &Event,
    scopes: impl IntoIterator<Item = &'t EventScope>,
    extra_config: &ExtraConfig,
) -> Config {
    let mut perf_event_attr = PerfEventAttr(RawPerfEventAttr {
        type_: 0,
        size: size_of::<RawPerfEventAttr>() as _,
        config: 0,
        // not use in counting mode
        __bindgen_anon_1: perf_event_attr__bindgen_ty_1::default(),
        sample_type: 0, // ditto
        // `Counter::new` or `CounterGroup::add_member` will clone this struct
        // and set `read_format` for their read format
        read_format: 0,
        _bitfield_align_1: [],
        // set later via perf_event_attr.set_...
        _bitfield_1: __BindgenBitfieldUnit::new([0u8; 8usize]),
        __bindgen_anon_2: perf_event_attr__bindgen_ty_2::default(), // not use in counting mode

        // The following 3 items are later set through event.enable_in_raw_attr...
        bp_type: 0,
        __bindgen_anon_3: perf_event_attr__bindgen_ty_3::default(),
        __bindgen_anon_4: perf_event_attr__bindgen_ty_4::default(),

        branch_sample_type: 0, // not use in counting mode
        sample_regs_user: 0,   // ditto
        sample_stack_user: 0,  // ditto
        #[cfg(feature = "linux-4.1")]
        clockid: 0, // ditto
        #[cfg(feature = "linux-3.19")]
        sample_regs_intr: 0, // ditto
        #[cfg(feature = "linux-4.1")]
        aux_watermark: 0, // ditto
        #[cfg(feature = "linux-4.8")]
        sample_max_stack: 0, // ditto
        __reserved_2: 0,
        #[cfg(feature = "linux-5.5")]
        aux_sample_size: 0, // not use in counting mode
        //#[cfg(feature = "linux-5.5")]
        //__reserved_3: 0,
        #[cfg(feature = "linux-5.13")]
        sig_data: 0, // not use in counting mode

        // TODO: https://github.com/torvalds/linux/commit/09519ec3b19e4144b5f6e269c54fbb9c294a9fcb
        #[cfg(feature = "linux-6.3")]
        config3: 0,

        ..Default::default() // TODO: skipped `__reserved_3` to build on 6.13 kernel
    });

    perf_event_attr.set_disabled(1);
    perf_event_attr.set_inherit(extra_config.inherit as _);
    perf_event_attr.set_pinned(extra_config.pinned as _);
    perf_event_attr.set_exclusive(extra_config.exclusive as _);

    perf_event_attr.set_exclude_user(1);
    perf_event_attr.set_exclude_kernel(1);
    perf_event_attr.set_exclude_hv(1);
    perf_event_attr.set_exclude_idle(1);

    perf_event_attr.set_mmap(0); // not use in counting mode
    perf_event_attr.set_comm(0); // ditto
    perf_event_attr.set_freq(0); // ditto
    perf_event_attr.set_inherit_stat(extra_config.inherit_stat as _);
    perf_event_attr.set_enable_on_exec(extra_config.enable_on_exec as _);
    perf_event_attr.set_task(0); // not use in counting mode
    perf_event_attr.set_watermark(0); // ditto
    perf_event_attr.set_precise_ip(0); // ditto
    perf_event_attr.set_mmap_data(0); // ditto
    perf_event_attr.set_sample_id_all(0); // ditto

    perf_event_attr.set_exclude_host(1);
    perf_event_attr.set_exclude_guest(1);

    perf_event_attr.set_exclude_callchain_kernel(1);
    perf_event_attr.set_exclude_callchain_user(1);

    #[cfg(feature = "linux-3.12")]
    perf_event_attr.set_mmap2(0); // not use in counting mode
    #[cfg(feature = "linux-3.16")]
    perf_event_attr.set_comm_exec(0); // ditto
    #[cfg(feature = "linux-4.1")]
    perf_event_attr.set_use_clockid(0); // ditto
    #[cfg(feature = "linux-4.3")]
    perf_event_attr.set_context_switch(0); // ditto

    // The `write_backward` was first added to the Linux kernel in 4.7
    // the man documentation incorrectly says "since Linux 4.6"
    // See: https://github.com/torvalds/linux/commit/9ecda41acb971ebd07c8fb35faf24005c0baea12
    #[cfg(feature = "linux-4.7")]
    perf_event_attr.set_write_backward(0); // ditto

    #[cfg(feature = "linux-4.12")]
    perf_event_attr.set_namespaces(0); // ditto
    #[cfg(feature = "linux-5.1")]
    perf_event_attr.set_ksymbol(0); // ditto
    #[cfg(feature = "linux-5.1")]
    perf_event_attr.set_bpf_event(0); // ditto
    #[cfg(feature = "linux-5.4")]
    perf_event_attr.set_aux_output(0); // ditto
    #[cfg(feature = "linux-5.7")]
    perf_event_attr.set_cgroup(0); // ditto
    #[cfg(feature = "linux-5.9")]
    perf_event_attr.set_text_poke(0); // ditto
    #[cfg(feature = "linux-5.12")]
    perf_event_attr.set_build_id(0); // ditto
    #[cfg(feature = "linux-5.13")]
    perf_event_attr.set_inherit_thread(extra_config.inherit_thread as _);
    #[cfg(feature = "linux-5.13")]
    perf_event_attr.set_remove_on_exec(extra_config.remove_on_exec as _);
    #[cfg(feature = "linux-5.13")]
    perf_event_attr.set_sigtrap(0); // not use in counting mode

    event.enable_in_raw_attr(&mut perf_event_attr);

    scopes
        .into_iter()
        .for_each(|scope| scope.enable_in_raw_attr(&mut perf_event_attr));

    let kprobe_func_or_uprobe_path = match event {
        #[cfg(feature = "linux-4.17")]
        Event::DynamicPmu(DynamicPmuEvent::Kprobe {
            cfg: KprobeConfig::FuncAndOffset { kprobe_func, .. },
            ..
        }) => Some(kprobe_func.clone()),
        #[cfg(feature = "linux-4.17")]
        Event::DynamicPmu(DynamicPmuEvent::Uprobe {
            cfg: UprobeConfig { uprobe_path, .. },
            ..
        }) => Some(uprobe_path.clone()),
        _ => None,
    };

    Config {
        kprobe_func_or_uprobe_path,
        perf_event_attr,
    }
}
