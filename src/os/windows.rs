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
// pub type BTH_ADDR = ULONGLONG;
pub type HBLUETOOTH_DEVICE_FIND = LPVOID;
pub type HBLUETOOTH_RADIO_FIND = LPVOID;
STRUCT!{struct BLUETOOTH_ADDRESS{
    inner: ULONGLONG,
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
    // pub fn BluetoothSelectDevices (
    //     pbtsdp: *mut BLUETOOTH_SELECT_DEVICE_PARAMS
    // ) -> BOOL;
    // pub fn BluetoothSelectDevicesFree (
    //     pbtsdp: *mut BLUETOOTH_SELECT_DEVICE_PARAMS
    // ) -> BOOL;
    // pub fn BluetoothFindFirstDevice (
    //     pbtsp: *const BLUETOOTH_DEVICE_SEARCH_PARAMS,
    //     pbtdi: *mut BLUETOOTH_DEVICE_INFO,
    // ) -> HBLUETOOTH_DEVICE_FIND;
    // pub fn BluetoothFindNextDevice (
    //     hFind: HBLUETOOTH_DEVICE_FIND,
    //     pbtdi: *mut BLUETOOTH_DEVICE_INFO,
    // ) -> BOOL;
    // pub fn BluetoothFindDeviceClose (
    //     hFind: HBLUETOOTH_DEVICE_FIND
    // ) -> BOOL;
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
    // pub fn BluetoothIsDiscoverable(
    //     hRadio: HANDLE,
    // ) -> BOOL;
}

macro_rules! create_struct {
    ($struct_name: ident, $ptr_name: ident, $struct_type: ty) => {
        let mut $struct_name: $struct_type = core::mem::zeroed();
        $struct_name.dwSize = core::mem::size_of::<$struct_type>() as DWORD;
        let $ptr_name = &$struct_name as *const _ as *mut $struct_type;
    };
}

#[derive(Debug)]
pub struct Device {

}

impl Device {
    // pub fn info() -> DeviceInfo {
    //     unimplemented!()
    // }
}

// #[derive(Debug)]
// pub struct DeviceInfo {

// }

// impl DeviceInfo {
//     pub fn addr(&self) -> Addr {
//         unimplemented!()
//     }

//     pub fn class(&self) -> Class {
//         unimplemented!()
//     }

//     pub fn connected(&self) -> bool {
//         unimplemented!()
//     }

//     pub fn remembered(&self) -> bool {
//         unimplemented!()
//     }

//     pub fn authenticated(&self) -> bool {
//         unimplemented!()
//     }

//     pub fn last_seen(&self) -> Instant {
//         unimplemented!()
//     }

//     pub fn last_used(&self) -> Instant {
//         unimplemented!()
//     }

//     pub fn name(&self) -> String {
//         unimplemented!()
//     }
// }

#[derive(Debug)]
pub struct Devices {
    hFindDevice: HBLUETOOTH_DEVICE_FIND,
}

pub struct SearchOptions {
    btsp: BLUETOOTH_DEVICE_SEARCH_PARAMS,
    pbtsp: *mut BLUETOOTH_DEVICE_SEARCH_PARAMS,
}

impl SearchOptions {
    pub fn new() -> Self {
        unsafe { 
            create_struct!(btsp, pbtsp, BLUETOOTH_DEVICE_SEARCH_PARAMS); 
            Self { btsp, pbtsp }
        }
    }

    pub fn return_authenticated(&mut self, authenticated: bool) -> &mut Self {
        self.btsp.fReturnAuthenticated = authenticated as BOOL;
        self
    }

    pub fn return_connected(&mut self, connected: bool) -> &mut Self {
        self.btsp.fReturnConnected = connected as BOOL;
        self
    }

    pub fn return_remembered(&mut self, remembered: bool) -> &mut Self {
        self.btsp.fReturnRemembered = remembered as BOOL;
        self
    }

    pub fn return_unknown(&mut self, unknown: bool) -> &mut Self {
        self.btsp.fReturnUnknown = unknown as BOOL;
        self
    }

    pub fn issue_inquiry(&mut self, issue_inquiry: bool) -> &mut Self {
        self.btsp.fIssueInquiry = issue_inquiry as BOOL;
        self
    }

    pub fn timeout_multiplier(&mut self, timeout_multiplier: u8) -> &mut Self {
        self.btsp.cTimeoutMultiplier = timeout_multiplier as UCHAR;
        self
    }

    pub fn search(&self, radio: &Radio) -> Devices {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Radio {
    hRadio: HANDLE,
}

fn ull_to_addr(src: ULONGLONG) -> crate::Addr {
    crate::Addr::from(unsafe { *(&src as *const u64 as *const [u8; 6]) })
}

impl Radio {
    fn new(hRadio: HANDLE) -> Self {
        Self { hRadio }
    }

    pub fn info(&self) -> crate::RadioInfo {
        unsafe { 
            create_struct!(radioInfo, pRadioInfo, BLUETOOTH_RADIO_INFO);
            BluetoothGetRadioInfo(self.hRadio, pRadioInfo);
            let mut buf = core::mem::zeroed::<crate::RadioInfo>();
            buf.addr = ull_to_addr(radioInfo.address.inner);
            buf.name = String::from_utf16_lossy(&radioInfo.szName).trim_end_matches(|c| c=='\0').to_string();
            buf.class = (radioInfo.ulClassofDevice as u32).into();
            buf.subversion = radioInfo.lmpSubversion as u16;
            buf.manufacturer = radioInfo.manufacturer as u16;
            buf
        }
    }

    pub fn devices(&self) -> Devices {
        unimplemented!()
    } 

    // pub fn is_discoverable(&self) -> bool {
    //     unsafe {
    //         BluetoothIsDiscoverable(self.hRadio) == TRUE
    //     }
    // }
}

impl Drop for Radio {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.hRadio);
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
