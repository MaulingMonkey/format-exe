use super::*;



/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory#remarks>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DataDirectories {
    /// IMAGE_DIRECTORY_ENTRY_EXPORT
    pub export:             DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_IMPORT
    pub import:             DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_RESOURCE
    pub resource:           DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_EXCEPTION
    pub exception:          DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_SECURITY
    /// Certificates related stuff
    pub security:           DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_BASERELOC
    /// Base relocation table
    pub basereloc:          DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_DEBUG
    pub debug:              DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_ARCHITECTURE
    /// Architecture-specific data
    ///
    /// IMAGE_DIRECTORY_ENTRY_COPYRIGHT (x86 usage)
    pub architecture:       DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_GLOBALPTR
    ///
    /// Global pointer register relative virtual address
    pub globalptr:          DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_TLS
    /// Thread local storage (TLS)
    pub tls:                DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG
    pub load_config:        DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT
    pub bound_imports:      DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_IAT
    pub iat:                DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT
    pub delay_import:       DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR
    /// COM runtime descriptor / CLR header
    pub com_descriptor:     DataDirectory,

    _reserved:              DataDirectory,
}

impl DataDirectories {
    pub fn iter_name_dd<'a>(&'a self) -> impl Iterator<Item = (&'static str, &'a DataDirectory)> {
        std::array::IntoIter::new([
            ("export",          &self.export            ),
            ("import",          &self.import            ),
            ("resource",        &self.resource          ),
            ("exception",       &self.exception         ),
            ("security",        &self.security          ),
            ("basereloc",       &self.basereloc         ),
            ("debug",           &self.debug             ),
            ("architecture",    &self.architecture      ),
            ("globalptr",       &self.globalptr         ),
            ("tls",             &self.tls               ),
            ("load_config",     &self.load_config       ),
            ("bound_imports",   &self.bound_imports     ),
            ("iat",             &self.iat               ),
            ("delay_import",    &self.delay_import      ),
            ("com_descriptor",  &self.com_descriptor    ),
        ])
    }
}

impl From<RawDataDirectories> for DataDirectories {
    fn from(value: RawDataDirectories) -> Self {
        let [
            export, import, resource, exception,
            security, basereloc, debug, architecture,
            globalptr, tls, load_config, bound_imports,
            iat, delay_import, com_descriptor, _reserved,
        ] = value;

        Self {
            export:                 export.into(),
            import:                 import.into(),
            resource:               resource.into(),
            exception:              exception.into(),
            security:               security.into(),
            basereloc:              basereloc.into(),
            debug:                  debug.into(),
            architecture:           architecture.into(),
            globalptr:              globalptr.into(),
            tls:                    tls.into(),
            load_config:            load_config.into(),
            bound_imports:          bound_imports.into(),
            iat:                    iat.into(),
            delay_import:           delay_import.into(),
            com_descriptor:         com_descriptor.into(),
            _reserved:              _reserved.into(),
        }
    }
}

const IMAGE_NUMBEROF_DIRECTORY_ENTRIES : usize = 16;
pub(crate) type RawDataDirectories = [RawDataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES];

#[test] fn layout() {
    use std::mem::*;

    assert_eq!(size_of::<DataDirectories>(), size_of::<[DataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES]>());
    assert_eq!(align_of::<DataDirectories>(), align_of::<[DataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES]>());
}
