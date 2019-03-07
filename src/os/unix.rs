
#[link(name = "bluetooth")]
extern "C" {
    fn hci_get_route(addr: *mut BtAddr) -> c_int;
    fn hci_open_dev(device_id: c_int) -> c_int;
    fn hci_inquiry(device_id: c_int, timeout: c_int, max_rsp: c_int, lap: *const u8, inquiry_info: *mut *mut InquiryInfo, flags: c_long) -> c_int;

    fn hci_read_remote_name(socket: c_int, addr: *const BtAddr, max_len: c_int, name: *mut c_char, timeout_ms: c_int) -> c_int;
}
