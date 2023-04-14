use super::*;



from_memory_struct! {
    /// {
    ///     [export](struct.DataDirectories.html#structfield.export),
    ///     [import](struct.DataDirectories.html#structfield.import),
    ///     [resource](struct.DataDirectories.html#structfield.resource),
    ///     [exception](struct.DataDirectories.html#structfield.exception),
    ///     ...
    /// } &nbsp;&nbsp;&nbsp;&nbsp; Describes memory regions of the loaded executable.
    ///
    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory#remarks>
    /// *   <https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#optional-header-data-directories-image-only>
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
}

impl DataDirectories {
    pub fn iter_name_dd<'a>(&'a self) -> impl Iterator<Item = (&'static str, &'a DataDirectory)> {
        IntoIterator::into_iter([
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

    pub const EMPTY : DataDirectories = DataDirectories {
        export:         DataDirectory::EMPTY,
        import:         DataDirectory::EMPTY,
        resource:       DataDirectory::EMPTY,
        exception:      DataDirectory::EMPTY,
        security:       DataDirectory::EMPTY,
        basereloc:      DataDirectory::EMPTY,
        debug:          DataDirectory::EMPTY,
        architecture:   DataDirectory::EMPTY,
        globalptr:      DataDirectory::EMPTY,
        tls:            DataDirectory::EMPTY,
        load_config:    DataDirectory::EMPTY,
        bound_imports:  DataDirectory::EMPTY,
        iat:            DataDirectory::EMPTY,
        delay_import:   DataDirectory::EMPTY,
        com_descriptor: DataDirectory::EMPTY,
        _reserved:      DataDirectory::EMPTY,
    };
}


#[test] fn layout() {
    use std::mem::*;

    const IMAGE_NUMBEROF_DIRECTORY_ENTRIES : usize = 16;
    assert_eq!(size_of::<DataDirectories>(), size_of::<[DataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES]>());
    assert_eq!(align_of::<DataDirectories>(), align_of::<[DataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES]>());
}
