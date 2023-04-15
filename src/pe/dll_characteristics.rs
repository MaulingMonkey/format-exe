#[cfg(doc)] use crate::pe::*;

from_memory_flags! {
    /// [NX_COMPAT](Self::NX_COMPAT) |
    /// [NO_ISOLATION](Self::NO_ISOLATION) |
    /// [NO_SEH](Self::NO_SEH) |
    /// ...<br>
    /// [pe](Reader).[pe_header()](Header).[optional_header](Header::optional_header).unwrap().[dll_characteristics()](OptionalHeader::dll_characteristics): various flags/metadata<br>
    /// <br>
    ///
    /// ## References
    /// *   [pe::OptionalHeader32::dll_characteristics](crate::pe::OptionalHeader32::dll_characteristics)
    /// *   [pe::OptionalHeader64::dll_characteristics](crate::pe::OptionalHeader64::dll_characteristics)
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32>
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header64>
    /// *   `IMAGE_DLLCHARACTERISTICS_*`
    #[repr(transparent)]
    pub struct DllCharacteristics : u16 {
        const NONE                                  = 0;

        // IMAGE_LIBRARY_*
        #[doc(hidden)] const PROCESS_INIT           = 0x0001;
        #[doc(hidden)] const PROCESS_TERM           = 0x0002;
        #[doc(hidden)] const THREAD_INIT            = 0x0004;
        #[doc(hidden)] const THREAD_TERM            = 0x0008;

        #[doc(hidden)] const RESERVED_0010          = 0x0010;

        /// ASLR with 64 bit address space.
        ///
        /// ## References
        /// *   [/HIGHENTROPYVA (Support 64-Bit ASLR)](https://docs.microsoft.com/en-us/cpp/build/reference/highentropyva-support-64-bit-aslr)
        /// *   [Address Space Layout Randomization (ASLR)](https://docs.microsoft.com/en-us/previous-versions/bb430720(v=msdn.10)#address-space-layout-randomization-aslr)
        /// *   <https://en.wikipedia.org/wiki/Address_space_layout_randomization>
        const HIGH_ENTROPY_VA                       = 0x0020;

        /// The DLL can be relocated at load time.
        ///
        /// ## References
        /// *   <https://devblogs.microsoft.com/oldnewthing/20170120-00/?p=95225>
        /// *   <https://docs.microsoft.com/en-us/cpp/build/reference/dynamicbase-use-address-space-layout-randomization>
        const DYNAMIC_BASE                          = 0x0040;

        /// Code integrity checks are forced.
        ///
        /// ## References
        /// *   <https://docs.microsoft.com/en-us/cpp/build/reference/integritycheck-require-signature-check>
        /// *   <https://social.technet.microsoft.com/wiki/contents/articles/255.forced-integrity-signing-of-portable-executable-pe-files.aspx>
        const FORCE_INTEGRITY                       = 0x0080;

        /// The image is compatible with data execution prevention (DEP).
        ///
        /// ## References
        /// *   <https://docs.microsoft.com/en-us/windows/win32/memory/data-execution-prevention>
        /// *   <https://en.wikipedia.org/wiki/Executable_space_protection#Windows>
        const NX_COMPAT                             = 0x0100;

        /// The image is isolation aware, but should not be isolated.
        ///
        /// ## References
        /// *   <https://docs.microsoft.com/en-us/windows/win32/sbscs/isolated-applications-and-side-by-side-assemblies-portal>
        /// *   <https://docs.microsoft.com/en-us/windows/win32/sbscs/isolating-components>
        const NO_ISOLATION                          = 0x0200;

        /// The image does not use structured exception handling (SEH)..
        ///
        /// ## References
        /// *   <https://docs.microsoft.com/en-us/cpp/cpp/structured-exception-handling-c-cpp>
        /// *   <https://en.wikipedia.org/wiki/Microsoft-specific_exception_handling_mechanisms#Structured_Exception_Handling>
        const NO_SEH                                = 0x0400;

        /// Do not bind the image.
        ///
        /// ## References
        /// *   <https://docs.microsoft.com/en-us/cpp/build/reference/allowbind-prevent-dll-binding>
        /// *   <https://reverseengineering.stackexchange.com/questions/22013/what-does-bindimageex-actually-do>
        /// *   <https://www.codeproject.com/Articles/9468/Need-for-Binding-an-Executable-to-DLLs>
        const NO_BIND                               = 0x0800;

        /// Image should execute in an AppContainer.
        ///
        /// ## References
        /// *   <https://docs.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation>
        /// *   <https://blahcat.github.io/2020/12/30/cheap_sandboxing_with_appcontainers/>
        /// *   <https://github.com/microsoft/WindowsAppSDK/issues/219>
        const APPCONTAINER                          = 0x1000;

        /// A WDM driver.
        ///
        /// ## References
        /// *   <https://en.wikipedia.org/wiki/Windows_Driver_Model>
        /// *   <https://docs.microsoft.com/en-us/windows-hardware/drivers/kernel/writing-wdm-drivers>
        /// *   <https://docs.microsoft.com/en-us/windows-hardware/drivers/kernel/introduction-to-wdm>
        const WDM_DRIVER                            = 0x2000;

        /// Image supports Control Flow Guard (CFG).
        ///
        /// ## References
        /// *   <https://docs.microsoft.com/en-us/windows/win32/secbp/control-flow-guard>
        const GUARD_CF                              = 0x4000;

        /// The image is terminal server aware.
        ///
        /// ## References
        /// *   <https://docs.microsoft.com/en-us/cpp/build/reference/tsaware-create-terminal-server-aware-application?view=msvc-160>
        const TERMINAL_SERVER_AWARE                 = 0x8000;
    }
}
