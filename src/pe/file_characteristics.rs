#[cfg(doc)] use super::*;

from_memory_flags! {
    /// [EXECUTABLE_IMAGE](Self::EXECUTABLE_IMAGE) |
    /// [LARGE_ADDRESS_AWARE](Self::LARGE_ADDRESS_AWARE) |
    /// [DLL](Self::DLL) |
    /// ...<br>
    /// [pe](Reader).[pe_header()](Header).[file_header](FileHeader).[characteristics](FileHeader::characteristics): various flags/metadata<br>
    /// <br>
    ///
    /// | Example           | Machine   | Value |
    /// | ------------------| ----------| ------|
    /// | `notepad.exe`     | AMD64     | <code>EXECUTABLE_IMAGE \| LARGE_ADDRESS_AWARE</code>
    /// | `xinput1_4.dll`   | AMD64     | <code>EXECUTABLE_IMAGE \| LARGE_ADDRESS_AWARE \| DLL</code>
    /// | `steam.exe`       | x86       | <code>EXECUTABLE_IMAGE \| LARGE_ADDRESS_AWARE \| MACHINE_32BIT</code>
    /// | `winamp.exe`      | x86       | <code>EXECUTABLE_IMAGE \| LARGE_ADDRESS_AWARE \| MACHINE_32BIT</code>
    ///
    /// ## References
    /// *   [pe::FileHeader::characteristics](crate::pe::FileHeader::characteristics)
    /// *   <https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_file_header#members> (see "Characteristics" field)
    /// *   `IMAGE_FILE_*`
    #[repr(transparent)]
    pub struct FileCharacteristics : u16 {
        /// Zero / no flags set.
        const NONE                      = 0;

        /// Relocation information was stripped from the file.
        /// The file must be loaded at its preferred base address.
        const RELOCS_STRIPPED           = 0x0001;

        /// The file is "executable" (≈ has no unresolved external references - includes [`DLL`](Self::DLL)s)
        const EXECUTABLE_IMAGE          = 0x0002;

        /// [COFF](https://en.wikipedia.org/wiki/COFF#Symbolic_debugging_information) line numbers were stripped
        const LINE_NUMS_STRIPPED        = 0x0004;

        /// [COFF](https://en.wikipedia.org/wiki/COFF#Symbolic_debugging_information) symbol table entries were stripped
        const LOCAL_SYMS_STRIPPED       = 0x0008;

        /// "Aggressively trim the working set" (obselete memory optimization hint?)
        const AGGRESSIVE_WS_TRIM        = 0x0010;

        /// The application can handle addresses larger than 2 GB.
        /// Typically set for 64-bit executables, rarely set for 32-bit executables.
        ///
        /// ## See Also
        /// *   [`/LARGEADDRESSAWARE`](https://learn.microsoft.com/en-us/cpp/build/reference/largeaddressaware-handle-large-addresses)
        /// *   [Memory Management - Demystifying `/3GB`](https://techcommunity.microsoft.com/t5/ask-the-performance-team/memory-management-demystifying-3gb/ba-p/372333)
        const LARGE_ADDRESS_AWARE       = 0x0020;

        #[doc(hidden)] const RESERVED_0040 = 0x0040;

        /// "The bytes of the word are reversed." (obselete flag... [PowerPC](https://en.wikipedia.org/wiki/PowerPC) related?)
        const BYTES_REVERSED_LO         = 0x0080;

        /// The computer supports 32-bit words.  (Not set for 64-bit binaries)
        const MACHINE_32BIT             = 0x0100;

        /// Debugging information was removed (and [stored separately](https://en.wikipedia.org/wiki/Program_database) in another file?)
        const DEBUG_STRIPPED            = 0x0200;

        /// If the image is on [removable media](https://en.wikipedia.org/wiki/CD-ROM), copy it to and run it from the swap file.
        const REMOVABLE_RUN_FROM_SWAP   = 0x0400;

        /// If the image is on the network, copy it to and run it from the swap file.
        const NET_RUN_FROM_SWAP         = 0x0800;

        /// "The image is a system file." (relates to e.g. Microsoft Windows?)
        const SYSTEM                    = 0x1000;

        /// The image is a [DLL](https://en.wikipedia.org/wiki/Dynamic-link_library). While it is an executable file, it cannot be run directly.
        const DLL                       = 0x2000;

        /// The file should be run only on a uniprocessor computer.
        const UP_SYSTEM_ONLY            = 0x4000;

        /// "The bytes of the word are reversed." (obselete flag... [PowerPC](https://en.wikipedia.org/wiki/PowerPC) related?)
        const BYTES_REVERSED_HI         = 0x8000;
    }
}
