bitflags::bitflags! {
    /// IMAGE_DLLCHARACTERISTICS_\*
    ///
    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32>
    #[repr(transparent)]
    pub struct DllCharacteristics : u16 {
        const NONE                                  = 0;

        // IMAGE_LIBRARY_*
        #[doc(hidden)] const PROCESS_INIT           = 0x0001;
        #[doc(hidden)] const PROCESS_TERM           = 0x0002;
        #[doc(hidden)] const THREAD_INIT            = 0x0004;
        #[doc(hidden)] const THREAD_TERM            = 0x0008;

        #[doc(hidden)] const RESERVED_0010          = 0x0010;
        const HIGH_ENTROPY_VA                       = 0x0020;
        const DYNAMIC_BASE                          = 0x0040;
        const FORCE_INTEGRITY                       = 0x0080;

        const NX_COMPAT                             = 0x0100;
        const NO_ISOLATION                          = 0x0200;
        const NO_SEH                                = 0x0400;
        const NO_BIND                               = 0x0800;

        const APPCONTAINER                          = 0x1000;
        const WDM_DRIVER                            = 0x2000;
        const GUARD_CF                              = 0x4000;
        const TERMINAL_SERVER_AWARE                 = 0x8000;
    }
}

impl Default for DllCharacteristics {
    fn default() -> Self {
        Self::empty()
    }
}
