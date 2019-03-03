#![allow(non_camel_case_types, non_snake_case)]
use core::ptr::NonNull;
use winapi::{
    ctypes::c_void,
    shared::{
        minwindef::{BOOL, DWORD, ULONG, USHORT, UCHAR, LPVOID, TRUE},
        ntdef::{LPWSTR, LPCWSTR, WCHAR, ULONGLONG, HANDLE},
        windef::{HWND},
    },
    um::{
        minwinbase::{SYSTEMTIME},
        handleapi::{CloseHandle},
    },
    STRUCT
};
pub const BLUETOOTH_MAX_NAME_SIZE: usize = 248;
pub type PFN_DEVICE_CALLBACK = Option<unsafe extern "system" fn(
    pvParam: LPVOID,
    pDevice: *const BLUETOOTH_DEVICE_INFO,
)>;
pub type PBLUETOOTH_DEVICE_INFO = *mut BLUETOOTH_DEVICE_INFO;
pub type PBLUETOOTH_RADIO_INFO = *mut BLUETOOTH_RADIO_INFO;
pub type BTH_ADDR = ULONGLONG;
pub type HBLUETOOTH_DEVICE_FIND = LPVOID;
pub type HBLUETOOTH_RADIO_FIND = LPVOID;
STRUCT!{struct BLUETOOTH_ADDRESS{
    inner: DWORD,
}}
STRUCT!{struct BLUETOOTH_DEVICE_INFO{ 
    dwSize: DWORD,
    Address: BLUETOOTH_ADDRESS,
    ulClassofDevice: ULONG,
    fConnected: BOOL,
    fRemembered: BOOL,
    fAuthenticated: BOOL,
    stLastSeen: SYSTEMTIME,
    stLastUsed: SYSTEMTIME,
    szName: [WCHAR; BLUETOOTH_MAX_NAME_SIZE],
}}
STRUCT!{struct BLUETOOTH_COD_PAIRS{
    ulCODMask: ULONG,
    pcszDescription: LPCWSTR,
}}
STRUCT!{struct BLUETOOTH_SELECT_DEVICE_PARAMS {
    dwSize: DWORD,
    cNumOfClasses: ULONG,
    prgClassOfDevices: *mut BLUETOOTH_COD_PAIRS,
    pszInfo: LPWSTR,
    hwndParent: HWND,
    fForceAuthentication: BOOL,
    fShowAuthenticated: BOOL,
    fShowRemembered: BOOL,
    fShowUnknown: BOOL,
    fAddNewDeviceWizard: BOOL,
    fSkipServicesPage: BOOL,
    pfnDeviceCallback: PFN_DEVICE_CALLBACK,
    pvParam: LPVOID,
    cNumDevices: DWORD,
    pDevices: PBLUETOOTH_DEVICE_INFO,
}}
STRUCT!{struct BLUETOOTH_DEVICE_SEARCH_PARAMS {
    dwSize: DWORD,
    fReturnAuthenticated: BOOL,
    fReturnRemembered: BOOL,
    fReturnUnknown: BOOL,
    fReturnConnected: BOOL,
    fIssueInquiry: BOOL,
    cTimeoutMultiplier: UCHAR,
    hRadio: HANDLE,
}}
STRUCT!{struct BLUETOOTH_RADIO_INFO {
    dwSize: DWORD,
    address: BLUETOOTH_ADDRESS,
    szName: [WCHAR; BLUETOOTH_MAX_NAME_SIZE],
    ulClassofDevice: ULONG,
    lmpSubversion: USHORT,
    manufacturer: USHORT,
}}
STRUCT!{struct BLUETOOTH_FIND_RADIO_PARAMS {
    dwSize: DWORD,
}}

extern "system" {
    pub fn BluetoothSelectDevices (
        pbtsdp: *mut BLUETOOTH_SELECT_DEVICE_PARAMS
    ) -> BOOL;
    pub fn BluetoothSelectDevicesFree (
        pbtsdp: *mut BLUETOOTH_SELECT_DEVICE_PARAMS
    ) -> BOOL;
    pub fn BluetoothFindFirstDevice (
        pbtsp: *const BLUETOOTH_DEVICE_SEARCH_PARAMS,
        pbtdi: *mut BLUETOOTH_DEVICE_INFO,
    ) -> HBLUETOOTH_DEVICE_FIND;
    pub fn BluetoothFindNextDevice (
        hFind: HBLUETOOTH_DEVICE_FIND,
        pbtdi: *mut BLUETOOTH_DEVICE_INFO,
    ) -> BOOL;
    pub fn BluetoothFindDeviceClose (
        hFind: HBLUETOOTH_DEVICE_FIND
    ) -> BOOL;
    pub fn BluetoothFindFirstRadio (
        pbtfrp: *const BLUETOOTH_FIND_RADIO_PARAMS,
        phRadio: *mut HANDLE,
    ) -> HBLUETOOTH_RADIO_FIND;
    pub fn BluetoothFindNextRadio (
        hFind: HBLUETOOTH_RADIO_FIND,
        phRadio: *mut HANDLE,
    ) -> BOOL;
    pub fn BluetoothGetRadioInfo (
        hRadio: HANDLE,
        pRadioInfo: PBLUETOOTH_RADIO_INFO,
    ) -> DWORD;
    pub fn BluetoothFindRadioClose (
        hFind: HBLUETOOTH_RADIO_FIND
    ) -> BOOL;
}

#[derive(Debug)]
pub struct Radio {
    handle: HANDLE,
}

impl Radio {
    fn new(handle: HANDLE) -> Self {
        Self { handle }
    }
}

impl Drop for Radio {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

#[derive(Debug)]
pub struct Radios {
    last_radio: HANDLE,
    handle_find_radio: Option<NonNull<c_void>>, 
}

impl Iterator for Radios {
    type Item = Radio;

    fn next(&mut self) -> Option<Self::Item> {
        self.handle_find_radio.map(|handle| unsafe {
            let ans = Radio::new(self.last_radio.clone());
            let hFindRadio = handle.as_ptr();
            let phRadio = &self.last_radio as *const _ as *mut _;
            if BluetoothFindNextRadio(hFindRadio, phRadio) != TRUE {
                self.handle_find_radio = None;
            }
            ans
        })
    }
}

impl Drop for Radios {
    fn drop(&mut self) {
        self.handle_find_radio.map(|handle| unsafe {
            let hFindRadio = handle.as_ptr();
            BluetoothFindRadioClose(hFindRadio);
        });
    }
}

macro_rules! create_struct {
    ($struct_name: ident, $ptr_name: ident, $struct_type: ty) => {
        let mut $struct_name: $struct_type = core::mem::zeroed();
        $struct_name.dwSize = core::mem::size_of::<$struct_type>() as DWORD;
        let $ptr_name = &$struct_name as *const _ as *mut $struct_type;
    };
}

pub fn radios() -> Radios {
    unsafe {
        let hRadio: HANDLE = core::ptr::null_mut();
        let phRadio = &hRadio as *const _ as *mut _;
        create_struct!(btfrp, pbtfrp, BLUETOOTH_FIND_RADIO_PARAMS);
        let hFindRadio = BluetoothFindFirstRadio(pbtfrp, phRadio);
        // todo: GetLastError
        Radios {
            last_radio: hRadio,
            handle_find_radio: NonNull::new(hFindRadio),
        }
    }    
}
