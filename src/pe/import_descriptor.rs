use crate::*;
use pe::*;



from_memory_struct! {
    /// {
    ///     [import_lookup_table_rva](struct.ImportDescriptor.html#structfield.import_lookup_table_rva),
    ///     [time_date_stamp](struct.ImportDescriptor.html#structfield.time_date_stamp),
    ///     [forwarder_chain](struct.ImportDescriptor.html#structfield.forwarder_chain),
    ///     [dll_ascii_name_rva](struct.ImportDescriptor.html#structfield.dll_ascii_name_rva),
    ///     [iat_rva](struct.ImportDescriptor.html#structfield.iat_rva)
    /// }<br>
    /// Describes the import of a single DLL, such as `"KERNEL32.dll"`<br>
    /// <br>
    ///
    /// ## References
    /// *   [PE Format: Import Directory Table](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#import-directory-table) (learn.microsoft.com)
    /// *   [An In-Depth Look into the Win32 Portable Executable File Format, Part 2](https://learn.microsoft.com/en-us/archive/msdn-magazine/2002/march/inside-windows-an-in-depth-look-into-the-win32-portable-executable-file-format-part-2) (MSDN March 2002)
    /// *   [A dive into the PE file format - PE file structure - Part 5: PE Imports (Import Directory Table, ILT, IAT)](https://0xrick.github.io/win-internals/pe6/) (0xRick's Blog)
    /// *   `IMAGE_IMPORT_DESCRIPTOR` in `C:\Program Files (x86)\Windows Kits\10\Include\10.0.19041.0\um\winnt.h`
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct ImportDescriptor {
        /// [`RVA`] of the first <code>[pe]::[ImportLookupTableEntry]{[32](ImportLookupTableEntry32),[64](ImportLookupTableEntry64)}</code>
        pub import_lookup_table_rva:    RVA,
        /// [`TimeDate::UNIX_EPOCH`] unless [bound](https://devblogs.microsoft.com/oldnewthing/20100318-00/?p=14563), in which case it's the timestamp of the DLL that was expected to resolve this import.
        pub time_date_stamp:            TimeDate,
        /// Related to [forwarding exports](https://devblogs.microsoft.com/oldnewthing/?p=30473)?  `0` if not bound, `!0` if no forwarders?
        ///
        /// > This field relates to forwarding. Forwarding involves one DLL sending on references to one of its functions to another DLL.
        /// > For example, in Windows NT, NTDLL.DLL appears to forward some of its exported functions to KERNEL32.DLL.
        /// > An application may think it's calling a function in NTDLL.DLL, but it actually ends up calling into KERNEL32.DLL.
        /// > This field contains an index into FirstThunk array (described momentarily).
        /// > The function indexed by this field will be forwarded to another DLL.
        /// > Unfortunately, the format of how a function is forwarded isn't documented, and examples of forwarded functions are hard to find.
        /// >
        /// > [Peering Inside the PE: A Tour of the Win32 Portable Executable File Format](https://learn.microsoft.com/en-us/previous-versions/ms809762(v=msdn.10))
        /// > §&nbsp;[PE File Imports](https://learn.microsoft.com/en-us/previous-versions/ms809762(v=msdn.10)#pe-file-imports)
        pub forwarder_chain:            u32,
        /// [`RVA`] of the name of the DLL being imported (e.g. `"KERNEL32.dll"`)
        pub dll_ascii_name_rva:         RVA,
        /// [`RVA`] of the Import Address Table (≈ patched/bound/loaded [`pe::ImportLookupTableEntry`]s?)
        pub iat_rva:                    RVA,
    }
}

impl ImportDescriptor {
    // ...
}
