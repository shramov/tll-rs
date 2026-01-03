#![allow(non_upper_case_globals)]

use tll_sys::channel::*;

bitflags! {
    //#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] // Enable for bitflags >= 2.0
    pub struct Caps: u32 {
        const Input = TLL_CAPS_INPUT;
        const Output = TLL_CAPS_OUTPUT;
        const InOut = TLL_CAPS_INOUT;

        const Custom = TLL_CAPS_CUSTOM;
        const Parent = TLL_CAPS_PARENT;
        const Proxy = TLL_CAPS_PROXY;
    }
}

bitflags! {
    //#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] // Enable for bitflags >= 2.0
    pub struct DCaps: u32 {
        const POLLIN = TLL_DCAPS_POLLIN;
        const POLLOUT = TLL_DCAPS_POLLOUT;
        const POLLMASK = TLL_DCAPS_POLLMASK;

        const Process = TLL_DCAPS_PROCESS;
        const Pending = TLL_DCAPS_PENDING;
        const Suspend = TLL_DCAPS_SUSPEND;
        const SuspendPermanent = TLL_DCAPS_SUSPEND_PERMANENT;
    }
}
