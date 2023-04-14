from_memory_flags! {
    /// IMAGE_SCN_\*
    ///
    /// ## References
    /// *   <https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_section_header>
    #[repr(transparent)]
    pub struct SectionCharacteristics : u32 {
        const NONE                                  = 0;

        #[doc(hidden)] // Reserved
        const TYPE_REG               = 0x00000000;
        //#[doc(hidden)] // Replaced by SCALE_INDEX
        //const TYPE_DSECT           = 0x00000001;
        #[doc(hidden)] // Reserved
        const TYPE_NOLOAD            = 0x00000002;
        #[doc(hidden)] // Reserved
        const TYPE_GROUP             = 0x00000004;
        #[doc(hidden)] // Reserved
        const TYPE_NO_PAD            = 0x00000008;
        #[doc(hidden)] // Reserved
        const TYPE_COPY              = 0x00000010;


        /// The section **c**on**t**ai**n**s executable code
        const CNT_CODE                              = 0x00000020;

        /// The section **c**on**t**ai**n**s initialized data
        const CNT_INITIALIZED_DATA                  = 0x00000040;

        /// The section **c**on**t**ai**n**s uninitialized data
        const CNT_UNINITIALIZED_DATA                = 0x00000080;



        /// Reserved
        const LNK_OTHER                             = 0x00000100;

        /// The section contains comments or other information. This is valid only for object files.
        const LNK_INFO                              = 0x00000200;

        #[doc(hidden)] // Reserved
        const TYPE_OVER                             = 0x00000400;

        /// The section will not become part of the image. This is valid only for object files.
        const LNK_REMOVE                            = 0x00000800;

        /// The section contains [COMDAT](https://stackoverflow.com/a/2440933/953531) data. This is valid only for object files.
        const LNK_COMDAT                            = 0x00001000;



        #[doc(hidden)] // Reserved
        const RESERVED_00002000                     = 0x00002000;

        //#[doc(hidden)] // Replaced by NO_DEFER_SPEC_EXC
        //const MEM_PROTECTED                       = 0x00004000;

        /// Reset speculative exceptions handling bits in the TLB entries for this section.
        ///
        /// (Is this an anti-spectre mitigation of some sort?)
        const NO_DEFER_SPEC_EXC                     = 0x00004000;

        /// The section contains data referenced through the global pointer.
        const GPREL                                 = 0x00008000;

        const MEM_FARDATA                           = 0x00008000;

        #[doc(hidden)] // Reserved
        const MEM_SYSHEAP                           = 0x00010000;

        /// Reserved
        const MEM_PURGEABLE                         = 0x00020000;


        const MEM_16BIT                             = 0x00020000;

        /// Reserved
        const MEM_LOCKED                            = 0x00040000;

        /// Reserved
        const MEM_PRELOAD                           = 0x00080000;

        /// Align data on a boundary. These values are valid only for object files.
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

        /// The section contains extended relocations.
        /// The count of relocations for the section exceeds the 16 bits that is reserved for it in the section header.
        /// If the NumberOfRelocations field in the section header is 0xffff, the actual relocation count is stored in the VirtualAddress field of the first relocation.
        /// It is an error if IMAGE_SCN_LNK_NRELOC_OVFL is set and there are fewer than 0xffff relocations in the section.
        const LNK_NRELOC_OVFL                       = 0x01000000;

        /// The section can be discarded as needed.
        const MEM_DISCARDABLE                       = 0x02000000;

        /// The section cannot be cached.
        const MEM_NOT_CACHED                        = 0x04000000;

        /// The section cannot be paged.
        const MEM_NOT_PAGED                         = 0x08000000;

        /// The section can be shared in memory.
        const MEM_SHARED                            = 0x10000000;

        /// The section can be executed as code.
        const MEM_EXECUTE                           = 0x20000000;

        /// The section can be read.
        const MEM_READ                              = 0x40000000;

        /// The section can be written to.
        const MEM_WRITE                             = 0x80000000;

        /// TLS index is scaled
        const SCALE_INDEX                           = 0x00000001;
    }
}
