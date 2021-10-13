use crate::*;

use bytemuck::*;

use std::fmt::{self, *};



/// [WINDOWS_GUI](Self::WINDOWS_GUI),
/// [WINDOWS_CUI](Self::WINDOWS_CUI),
/// [EFI_APPLICATION](Self::EFI_APPLICATION),
/// ... &nbsp;&nbsp;&nbsp;&nbsp; Subsystems
///
/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32>
/// *   `IMAGE_SUBSYSTEM_*` in `C:\Program Files (x86)\Windows Kits\10\Include\10.0.19041.0\um\winnt.h`
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq)] // TODO: PartialOrd/Ord/Hash
#[derive(Pod, Zeroable)]
pub struct Subsystem(u16le);

impl Subsystem {
    pub const fn new(value: u16) -> Self {
        Self(u16le::new(value))
    }

    #[doc = "Unknown subsystem"                                                 ] pub const UNKNOWN                     : Subsystem = Subsystem::new(0);
    #[doc = "No subsystem required (device drivers / native system processes)"  ] pub const NATIVE                      : Subsystem = Subsystem::new(1);
    #[doc = "Windows graphical user interface"                                  ] pub const WINDOWS_GUI                 : Subsystem = Subsystem::new(2);
    #[doc = "Windows character-mode user interface"                             ] pub const WINDOWS_CUI                 : Subsystem = Subsystem::new(3);
    #[doc = "OS/2 character-mode user interface"                                ] pub const OS2_CUI                     : Subsystem = Subsystem::new(5);
    #[doc = "POSIX character-mode user interface"                               ] pub const POSIX_CUI                   : Subsystem = Subsystem::new(7);
    #[doc = "Windows CE graphical user interface"                               ] pub const WINDOWS_CE_GUI              : Subsystem = Subsystem::new(9);
    #[doc = "Extensible Firmware Interface application"                         ] pub const EFI_APPLICATION             : Subsystem = Subsystem::new(10);
    #[doc = "Extensible Firmware Interface driver w/ boot services"             ] pub const EFI_BOOT_SERVICE_DRIVER     : Subsystem = Subsystem::new(11);
    #[doc = "Extensible Firmware Interface driver w/ runtime services"          ] pub const EFI_RUNTIME_DRIVER          : Subsystem = Subsystem::new(12);
    #[doc = "Extensible Firmware Interface ROM image"                           ] pub const EFI_ROM                     : Subsystem = Subsystem::new(13);
    #[doc = "Xbox system"                                                       ] pub const XBOX                        : Subsystem = Subsystem::new(14);
    #[doc = "Boot application"                                                  ] pub const WINDOWS_BOOT_APPLICATION    : Subsystem = Subsystem::new(16);
    #[doc = "???"                                                               ] pub const XBOX_CODE_CATALOG           : Subsystem = Subsystem::new(17);
}

impl Debug for Subsystem {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            Self::UNKNOWN                   => write!(fmt, "Subsystem::UNKNOWN"),
            Self::NATIVE                    => write!(fmt, "Subsystem::NATIVE"),
            Self::WINDOWS_GUI               => write!(fmt, "Subsystem::WINDOWS_GUI"),
            Self::WINDOWS_CUI               => write!(fmt, "Subsystem::WINDOWS_CUI"),
            Self::OS2_CUI                   => write!(fmt, "Subsystem::OS2_CUI"),
            Self::POSIX_CUI                 => write!(fmt, "Subsystem::POSIX_CUI"),
            Self::WINDOWS_CE_GUI            => write!(fmt, "Subsystem::WINDOWS_CE_GUI"),
            Self::EFI_APPLICATION           => write!(fmt, "Subsystem::EFI_APPLICATION"),
            Self::EFI_BOOT_SERVICE_DRIVER   => write!(fmt, "Subsystem::EFI_BOOT_SERVICE_DRIVER"),
            Self::EFI_RUNTIME_DRIVER        => write!(fmt, "Subsystem::EFI_RUNTIME_DRIVER"),
            Self::EFI_ROM                   => write!(fmt, "Subsystem::EFI_ROM"),
            Self::XBOX                      => write!(fmt, "Subsystem::XBOX"),
            Self::WINDOWS_BOOT_APPLICATION  => write!(fmt, "Subsystem::WINDOWS_BOOT_APPLICATION"),
            Self::XBOX_CODE_CATALOG         => write!(fmt, "Subsystem::XBOX_CODE_CATALOG"),
            other                           => write!(fmt, "Subsystem(0x{:04x})", other.0.to_le()),
        }
    }
}
