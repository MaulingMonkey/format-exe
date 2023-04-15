use crate::*;

use bytemuck::*;

use std::fmt::{self, Debug, Formatter};



/// ∑
/// [I386](Self::I386) |
/// [AMD64](Self::AMD64) |
/// [IA64](Self::IA64) |
/// [ARM](Self::ARM) |
/// [ARM64](Self::ARM64) |
/// [THUMB](Self::THUMB) |
/// [POWERPC](Self::POWERPC) |
/// [EBC](Self::EBC) |
/// ...<br>
/// Machine/instruction architectures<br>
/// <br>
///
/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_file_header>
/// *   `IMAGE_FILE_MACHINE_*` in `C:\Program Files (x86)\Windows Kits\10\Include\10.0.19041.0\um\winnt.h`
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq)] // TODO: PartialOrd/Ord/Hash
#[derive(Pod, Zeroable)]
pub struct Machine(u16le);

impl Machine {
    pub const fn new(value: u16) -> Self {
        Self(u16le::new(value))
    }

    pub const UNKNOWN       : Machine = Machine::new(0);

    /// Useful for indicating we want to interact with the host and not a WoW guest.
    pub const TARGET_HOST   : Machine = Machine::new(0x0001);

    #[doc = "Intel 386 / x86"                       ] pub const I386          : Machine = Machine::new(0x014c);
    #[doc = "MIPS little-endian, 0x160 big-endian"  ] pub const R3000         : Machine = Machine::new(0x0162);
    #[doc = "MIPS little-endian"                    ] pub const R4000         : Machine = Machine::new(0x0166);
    #[doc = "MIPS little-endian"                    ] pub const R10000        : Machine = Machine::new(0x0168);
    #[doc = "MIPS little-endian WCE v2"             ] pub const WCEMIPSV2     : Machine = Machine::new(0x0169);
    #[doc = "Alpha_AXP"                             ] pub const ALPHA         : Machine = Machine::new(0x0184);
    #[doc = "SH3 little-endian"                     ] pub const SH3           : Machine = Machine::new(0x01a2);
    #[doc = ""                                      ] pub const SH3DSP        : Machine = Machine::new(0x01a3);
    #[doc = "SH3E little-endian"                    ] pub const SH3E          : Machine = Machine::new(0x01a4);
    #[doc = "SH4 little-endian"                     ] pub const SH4           : Machine = Machine::new(0x01a6);
    #[doc = "SH5"                                   ] pub const SH5           : Machine = Machine::new(0x01a8);
    #[doc = "ARM Little-Endian"                     ] pub const ARM           : Machine = Machine::new(0x01c0);
    #[doc = "ARM Thumb/Thumb-2 Little-Endian"       ] pub const THUMB         : Machine = Machine::new(0x01c2);
    #[doc = "ARM Thumb-2 Little-Endian"             ] pub const ARMNT         : Machine = Machine::new(0x01c4);
    #[doc = ""                                      ] pub const AM33          : Machine = Machine::new(0x01d3);
    #[doc = "IBM PowerPC Little-Endian"             ] pub const POWERPC       : Machine = Machine::new(0x01F0);
    #[doc = ""                                      ] pub const POWERPCFP     : Machine = Machine::new(0x01f1);
    #[doc = "Intel Itanium"                         ] pub const IA64          : Machine = Machine::new(0x0200);
    #[doc = "MIPS"                                  ] pub const MIPS16        : Machine = Machine::new(0x0266);
    #[doc = "ALPHA64"                               ] pub const ALPHA64       : Machine = Machine::new(0x0284);
    #[doc = "MIPS"                                  ] pub const MIPSFPU       : Machine = Machine::new(0x0366);
    #[doc = "MIPS"                                  ] pub const MIPSFPU16     : Machine = Machine::new(0x0466);
    #[doc = "ALPHA64"                               ] pub const AXP64         : Machine = Self::ALPHA64;
    #[doc = "Infineon"                              ] pub const TRICORE       : Machine = Machine::new(0x0520);
    #[doc = ""                                      ] pub const CEF           : Machine = Machine::new(0x0CEF);
    #[doc = "EFI Byte Code"                         ] pub const EBC           : Machine = Machine::new(0x0EBC);
    #[doc = "AMD64 (K8) / x64 / x86-64"             ] pub const AMD64         : Machine = Machine::new(0x8664);
    #[doc = "M32R little-endian"                    ] pub const M32R          : Machine = Machine::new(0x9041);
    #[doc = "ARM64 Little-Endian"                   ] pub const ARM64         : Machine = Machine::new(0xAA64);
    #[doc = ""                                      ] pub const CEE           : Machine = Machine::new(0xC0EE);
}

impl Debug for Machine {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        #[allow(unreachable_patterns)] // contains duplicates (AXP64)
        match *self {
            Self::UNKNOWN       => write!(fmt, "Machine::UNKNOWN"),
            Self::TARGET_HOST   => write!(fmt, "Machine::TARGET_HOST"),
            Self::I386          => write!(fmt, "Machine::I386"),
            Self::R3000         => write!(fmt, "Machine::R3000"),
            Self::R4000         => write!(fmt, "Machine::R4000"),
            Self::R10000        => write!(fmt, "Machine::R10000"),
            Self::WCEMIPSV2     => write!(fmt, "Machine::WCEMIPSV2"),
            Self::ALPHA         => write!(fmt, "Machine::ALPHA"),
            Self::SH3           => write!(fmt, "Machine::SH3"),
            Self::SH3DSP        => write!(fmt, "Machine::SH3DSP"),
            Self::SH3E          => write!(fmt, "Machine::SH3E"),
            Self::SH4           => write!(fmt, "Machine::SH4"),
            Self::SH5           => write!(fmt, "Machine::SH5"),
            Self::ARM           => write!(fmt, "Machine::ARM"),
            Self::THUMB         => write!(fmt, "Machine::THUMB"),
            Self::ARMNT         => write!(fmt, "Machine::ARMNT"),
            Self::AM33          => write!(fmt, "Machine::AM33"),
            Self::POWERPC       => write!(fmt, "Machine::POWERPC"),
            Self::POWERPCFP     => write!(fmt, "Machine::POWERPCFP"),
            Self::IA64          => write!(fmt, "Machine::IA64"),
            Self::MIPS16        => write!(fmt, "Machine::MIPS16"),
            Self::ALPHA64       => write!(fmt, "Machine::ALPHA64"),
            Self::MIPSFPU       => write!(fmt, "Machine::MIPSFPU"),
            Self::MIPSFPU16     => write!(fmt, "Machine::MIPSFPU16"),
            Self::AXP64         => write!(fmt, "Machine::AXP64"),
            Self::TRICORE       => write!(fmt, "Machine::TRICORE"),
            Self::CEF           => write!(fmt, "Machine::CEF"),
            Self::EBC           => write!(fmt, "Machine::EBC"),
            Self::AMD64         => write!(fmt, "Machine::AMD64"),
            Self::M32R          => write!(fmt, "Machine::M32R"),
            Self::ARM64         => write!(fmt, "Machine::ARM64"),
            Self::CEE           => write!(fmt, "Machine::CEE"),
            other               => write!(fmt, "Machine(0x{:04x})", other.0.to_le()),
        }
    }
}

impl FromMemory for Machine {
    type Raw    = Self;
    type Error  = std::io::Error;
    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> { Ok(raw) }
}
