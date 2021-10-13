bitflags::bitflags! {
    /// IMAGE_SCN_\*
    ///
    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32>
    #[repr(transparent)]
    pub struct SectionCharacteristics : u32 {
        const NONE                                  = 0;

        #[doc(hidden)] const TYPE_REG               = 0x00000000;
        //#[doc(hidden)] const TYPE_DSECT             = 0x00000001;
        #[doc(hidden)] const TYPE_NOLOAD            = 0x00000002;
        #[doc(hidden)] const TYPE_GROUP             = 0x00000004;
        #[doc(hidden)] const TYPE_NO_PAD            = 0x00000008;
        #[doc(hidden)] const TYPE_COPY              = 0x00000010;

        const CNT_CODE                              = 0x00000020;
        const CNT_INITIALIZED_DATA                  = 0x00000040;
        const CNT_UNINITIALIZED_DATA                = 0x00000080;

        const LNK_OTHER                             = 0x00000100;
        const LNK_INFO                              = 0x00000200;
        #[doc(hidden)] const TYPE_OVER              = 0x00000400;
        const LNK_REMOVE                            = 0x00000800;
        const LNK_COMDAT                            = 0x00001000;

        #[doc(hidden)] const RESERVED_00002000      = 0x00002000;
        //#[doc(hidden)] const MEM_PROTECTED          = 0x00004000;
        const NO_DEFER_SPEC_EXC                     = 0x00004000;
        const GPREL                                 = 0x00008000;
        const MEM_FARDATA                           = 0x00008000;
        #[doc(hidden)] const MEM_SYSHEAP            = 0x00010000;
        const MEM_PURGEABLE                         = 0x00020000;
        const MEM_16BIT                             = 0x00020000;
        const MEM_LOCKED                            = 0x00040000;
        const MEM_PRELOAD                           = 0x00080000;

        const ALIGN_MASK                            = 0x00F00000;
        const ALIGN_8192BYTES                       = 0x00E00000;
        const ALIGN_4096BYTES                       = 0x00D00000;
        const ALIGN_2048BYTES                       = 0x00C00000;
        const ALIGN_1024BYTES                       = 0x00B00000;
        const ALIGN_512BYTES                        = 0x00A00000;
        const ALIGN_256BYTES                        = 0x00900000;
        const ALIGN_128BYTES                        = 0x00800000;
        const ALIGN_64BYTES                         = 0x00700000;
        const ALIGN_32BYTES                         = 0x00600000;
        const ALIGN_16BYTES                         = 0x00500000; // Default alignment if no others are specified
        const ALIGN_8BYTES                          = 0x00400000;
        const ALIGN_4BYTES                          = 0x00300000;
        const ALIGN_2BYTES                          = 0x00200000;
        const ALIGN_1BYTES                          = 0x00100000;

        const LNK_NRELOC_OVFL                       = 0x01000000;
        const MEM_DISCARDABLE                       = 0x02000000;
        const MEM_NOT_CACHED                        = 0x04000000;
        const MEM_NOT_PAGED                         = 0x08000000;
        const MEM_SHARED                            = 0x10000000;
        const MEM_EXECUTE                           = 0x20000000;
        const MEM_READ                              = 0x40000000;
        const MEM_WRITE                             = 0x80000000;

        const SCALE_INDEX                           = 0x00000001;
    }
}

impl Default for SectionCharacteristics {
    fn default() -> Self {
        Self::empty()
    }
}
