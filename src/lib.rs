#![cfg(windows)]

use std::cell::{Cell, RefCell};
use std::net::UdpSocket;
use std::os::raw::*;
use binary_layout::define_layout;
use winapi::shared::minwindef::{BOOL, DWORD, HMODULE, LPVOID, TRUE};


mod ats_plugin;
use ats_plugin::*;

const ARRAY_LENGTH: usize = 256;

thread_local! {
    static POWER: Cell<c_int> = Cell::new(0);
    static BRAKE: Cell<c_int> = Cell::new(0);
    static REVERSER: Cell<c_int> = Cell::new(0);
    static SOCKET: RefCell<Option<UdpSocket>> = RefCell::new(None);
}

define_layout!(ELAPSE_PACKET, BigEndian, {
    speed: f32,
    time: i32,
});

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(_dll_module: HMODULE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_THREAD_ATTACH: DWORD = 2;
    const DLL_THREAD_DETACH: DWORD = 3;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => (),
        DLL_THREAD_ATTACH => (),
        DLL_THREAD_DETACH => (),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    TRUE
}

/// Called when this plug-in is loaded
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Load() {
    // Socketの作成
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    socket.connect("127.0.0.1:5901").unwrap();
    // Todo: Socketを別スレッドに移行
    SOCKET.set(Some(socket));
}

/// Called when this plug_in is unloaded
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Dispose() {
    // Socketの破棄
    SOCKET.set(None);
}

/// Returns the version numbers of ATS plug-in
#[no_mangle]
pub extern "system" fn GetPluginVersion() -> c_int {
    ATS_VERSION
}

/// Called when the train is loaded
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetVehicleSpec(_vehicle_spec: AtsVehicleSpec) {}

/// Called when the game is started
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Initialize(_brake: c_int) {}

/// Called every frame
///
/// # Safety
///
/// This function is marked as `unsafe` because it accesses the arrays pointed to by the argument
/// pointers. It is the caller's responsibility to make sure the arrays have 256 elements each and
/// have been properly initialized.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Elapse(
    vehicle_state: AtsVehicleState,
    p_panel: *mut c_int,
    p_sound: *mut c_int,
) -> AtsHandles {
    // Socketかチャンネル使ってデータ送る処理
    let speed = vehicle_state.speed as f32;
    let time = vehicle_state.time as i32;
    let f = |socket: &UdpSocket|{
        let mut data = [0u8; 8];
        data[0..4].clone_from_slice(&speed.to_be_bytes());
        data[4..8].clone_from_slice(&time.to_be_bytes());
        socket.send(&data).ok();
    };
    SOCKET.with_borrow(|socket|{
        socket.as_ref().map(f);
    });
    AtsHandles {
        brake: BRAKE.get(),
        power: POWER.get(),
        reverser: REVERSER.get(),
        constant_speed: ATS_CONSTANTSPEED_CONTINUE,
    }
}

/// Called when the power is changed
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetPower(notch: c_int) {
    POWER.set(notch);
}

/// Called when the brake is changed
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetBrake(notch: c_int) {
    BRAKE.set(notch);
}

/// Called when the reverser is changed
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetReverser(pos: c_int) {
    REVERSER.set(pos);
}

/// Called when any ATS key is pressed
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn KeyDown(_ats_key_code: c_int) {}

/// Called when any ATS key is released
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn KeyUp(_ats_key_code: c_int) {}

/// Called when the horn is used
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn HornBlow(_horn_type: c_int) {}

/// Called when the door is opened
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DoorOpen() {}

/// Called when the door is closed
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DoorClose() {}

/// Called when current signal is changed
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetSignal(_signal: c_int) {}

/// Called when the beacon data is received
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetBeaconData(_beacon_data: AtsBeaconData) {}
