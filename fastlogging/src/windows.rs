use windows_sys::{
    self,
    Win32::{
        Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
                TH32CS_SNAPPROCESS,
            },
            Threading::GetCurrentProcessId,
        },
    },
};

pub fn getppid() -> u32 {
    let h_snapshot: HANDLE = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if h_snapshot == INVALID_HANDLE_VALUE {
        return 0;
    }
    let mut pe32 = PROCESSENTRY32 {
        dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0u8; 260],
    };
    if !unsafe { Process32First(h_snapshot, &mut pe32) != 0 } {
        return 0;
    }
    let pid: u32 = unsafe { GetCurrentProcessId() };
    let mut ppid: u32 = 0;
    while unsafe { Process32Next(h_snapshot, &mut pe32) != 0 } {
        if pe32.th32ProcessID == pid {
            ppid = pe32.th32ParentProcessID;
            break;
        }
    }
    if h_snapshot != INVALID_HANDLE_VALUE {
        unsafe { CloseHandle(h_snapshot) };
    }
    ppid
}
