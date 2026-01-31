use anyhow::{Context, Result};
use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use coreaudio_sys::{
    kAudioDevicePropertyDeviceUID, kAudioHardwarePropertyDefaultInputDevice,
    kAudioHardwarePropertyDefaultOutputDevice, kAudioObjectPropertyElementMain,
    kAudioObjectPropertyScopeGlobal, kAudioObjectSystemObject, AudioDeviceID,
    AudioObjectGetPropertyData, AudioObjectPropertyAddress, OSStatus,
};
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;

/// CoreAudio の OSStatus を anyhow に落とす（最低限）
fn ensure_ok(status: OSStatus, msg: &'static str) -> Result<()> {
    if status == 0 {
        Ok(())
    } else {
        anyhow::bail!("{msg} (OSStatus={status})");
    }
}

/// デフォルト入出力デバイスの AudioDeviceID を取る
fn get_default_device_id(is_input: bool) -> Result<AudioDeviceID> {
    let selector = if is_input {
        kAudioHardwarePropertyDefaultInputDevice
    } else {
        kAudioHardwarePropertyDefaultOutputDevice
    };

    // system object から property を読む
    let address = AudioObjectPropertyAddress {
        mSelector: selector,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMain,
    };

    let mut device_id: AudioDeviceID = 0;
    let mut data_size = size_of::<AudioDeviceID>() as u32;

    let status = unsafe {
        AudioObjectGetPropertyData(
            kAudioObjectSystemObject,
            &address,
            0,
            ptr::null(),
            &mut data_size,
            (&mut device_id as *mut AudioDeviceID).cast::<c_void>(),
        )
    };

    ensure_ok(status, "failed to get default audio device id")?;
    Ok(device_id)
}

/// AudioDeviceID から DeviceUID（CFString）を取得して Rust String にする
fn get_device_uid(device_id: AudioDeviceID) -> Result<String> {
    let address = AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyDeviceUID,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMain,
    };

    // out は CFStringRef
    let mut cf_str_ref: core_foundation::string::CFStringRef = ptr::null_mut();
let mut data_size = size_of::<core_foundation::string::CFStringRef>() as u32;

let status = unsafe {
    AudioObjectGetPropertyData(
        device_id,
        &address,
        0,
        ptr::null(),
        &mut data_size,
        (&mut cf_str_ref as *mut core_foundation::string::CFStringRef) as *mut c_void,
    )
};


    ensure_ok(status, "failed to get device UID")?;
    if cf_str_ref.is_null() {
        anyhow::bail!("device UID CFStringRef is null");
    }

    // CoreFoundation の “Get Rule” で返ってくる想定なので wrap_under_get_rule を使う:contentReference[oaicite:4]{index=4}
    let cf_string = unsafe { CFString::wrap_under_get_rule(cf_str_ref) };
    Ok(cf_string.to_string())
}

/// quies が保存する “現在の AudioState（UID）” を取得する
pub fn current_audio_state() -> Result<(Option<String>, Option<String>)> {
    let out_id = get_default_device_id(false).context("get default output device")?;
    let in_id = get_default_device_id(true).context("get default input device")?;

    let out_uid = get_device_uid(out_id).context("get output device uid")?;
    let in_uid = get_device_uid(in_id).context("get input device uid")?;

    Ok((Some(out_uid), Some(in_uid)))
}
