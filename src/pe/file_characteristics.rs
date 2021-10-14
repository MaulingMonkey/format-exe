bitflags::bitflags! {
    #[repr(transparent)]
    pub struct FileCharacteristics : u16 {
        const NONE                      = 0;

        const RELOCS_STRIPPED           = 0x0001;
        const EXECUTABLE_IMAGE          = 0x0002;
        const LINE_NUMS_STRIPPED        = 0x0004;
        const LOCAL_SYMS_STRIPPED       = 0x0008;

        const AGGRESSIVE_WS_TRIM        = 0x0010;
        const LARGE_ADDRESS_AWARE       = 0x0020;
        #[doc(hidden)] const RESERVED_0040 = 0x0040;
        const BYTES_REVERSED_LO         = 0x0080;

        const MACHINE_32BIT             = 0x0100;
        const DEBUG_STRIPPED            = 0x0200;
        const REMOVABLE_RUN_FROM_SWAP   = 0x0400;
        const NET_RUN_FROM_SWAP         = 0x0800;

        const SYSTEM                    = 0x1000;
        const DLL                       = 0x2000;
        const UP_SYSTEM_ONLY            = 0x4000;
        const BYTES_REVERSED_HI         = 0x8000;
    }
}

impl Default for FileCharacteristics {
    fn default() -> Self {
        Self::empty()
    }
}
