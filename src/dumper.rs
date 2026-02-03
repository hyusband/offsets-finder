use sysinfo::{System};
use std::fs::File;
use std::io::Write;
use windows::Win32::System::Memory::{
    VirtualQueryEx, MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_READWRITE, PAGE_READONLY, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE
};
use windows::Win32::System::Threading::{
    OpenProcess, OpenProcessToken, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ
};
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::Foundation::{HANDLE, CloseHandle, LUID};
use windows::Win32::Security::{
    LookupPrivilegeValueW, AdjustTokenPrivileges, TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY,
    TOKEN_PRIVILEGES, LUID_AND_ATTRIBUTES, SE_PRIVILEGE_ENABLED
};
use windows::core::PCWSTR;

fn enable_debug_privilege() -> bool {
    unsafe {
        let mut token_handle = HANDLE::default();
        if OpenProcessToken(
            windows::Win32::System::Threading::GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token_handle
        ).is_err() {
            return false;
        }

        let mut luid = LUID::default();
        let name: Vec<u16> = "SeDebugPrivilege".encode_utf16().chain(std::iter::once(0)).collect();
        if LookupPrivilegeValueW(None, PCWSTR(name.as_ptr()), &mut luid).is_err() {
            let _ = CloseHandle(token_handle);
            return false;
        }

        let tp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        let result = AdjustTokenPrivileges(token_handle, false, Some(&tp), 0, None, None).is_ok();
        let _ = CloseHandle(token_handle);
        result
    }
}

pub fn dump_bluestacks_memory(output_path: &str) -> Result<(), String> {
    enable_debug_privilege();
    
    let mut system = System::new_all();
    system.refresh_all();

    let process = system.processes().values().find(|p| {
        let name = p.name().to_string_lossy().to_lowercase();
        name == "hd-player.exe" || name == "bluestacks.exe"
    }).ok_or("BlueStacks process not found (HD-Player.exe/BlueStacks.exe)")?;

    let pid_str = process.pid().to_string();
    let pid = pid_str.parse::<u32>().map_err(|_| "Failed to parse PID")?;
    println!("Found process: {} (PID: {})", process.name().to_string_lossy(), pid);

    unsafe {
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            pid
        ).map_err(|e| format!("Failed to open process: {}", e))?;

        let result = perform_dump(process_handle, output_path);
        
        let _ = CloseHandle(process_handle);
        result
    }
}

unsafe fn perform_dump(process_handle: HANDLE, output_path: &str) -> Result<(), String> {
    let mut file = File::create(output_path).map_err(|e| format!("Failed to create output file: {}", e))?;
    
    let mut address = 0;
    let mut total_dumped = 0;

    println!("Starting memory dump...");

    while address < 0x7FFFFFFFFFFF {
        let mut mbi = MEMORY_BASIC_INFORMATION::default();
        let size = VirtualQueryEx(
            process_handle,
            Some(address as *const _),
            &mut mbi,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>()
        );

        if size == 0 {
            break;
        }

        let is_readable = (mbi.Protect & PAGE_READONLY).0 != 0 ||
                          (mbi.Protect & PAGE_READWRITE).0 != 0 ||
                          (mbi.Protect & PAGE_EXECUTE_READ).0 != 0 ||
                          (mbi.Protect & PAGE_EXECUTE_READWRITE).0 != 0;

        if mbi.State == MEM_COMMIT && is_readable {
            let mut buffer = vec![0u8; mbi.RegionSize];
            let mut bytes_read = 0;
            
            let success = ReadProcessMemory(
                process_handle,
                mbi.BaseAddress,
                buffer.as_mut_ptr() as *mut _,
                mbi.RegionSize,
                Some(&mut bytes_read)
            ).is_ok();

            if success && bytes_read > 0 {
                file.write_all(&buffer[..bytes_read]).map_err(|e| format!("Failed to write to file: {}", e))?;
                total_dumped += bytes_read;
            }
        }

        address = mbi.BaseAddress as usize + mbi.RegionSize;
    }

    println!("Dump completed. Total bytes: {} MB", total_dumped / 1024 / 1024);
    Ok(())
}
