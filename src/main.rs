use std::{env, path::PathBuf};
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Security::*,
    Win32::{System::Threading::*, UI::WindowsAndMessaging::*},
};

fn run_window(file_path: String) -> Result<bool> {
    let si = STARTUPINFOW {
        cb: std::mem::size_of::<STARTUPINFOW>() as u32,
        ..Default::default()
    };

    let mut pi = PROCESS_INFORMATION {
        ..Default::default()
    };

    let pwd_result = env::current_dir();
    let pwd = pwd_result.unwrap();

    let ffplay = PathBuf::from("ffplay.exe");
    let file = PathBuf::from(file_path);

    let absolute_file = if file.is_absolute() {
        file
    } else {
        pwd.join(file)
    };

    let absolute_exe = pwd.join(ffplay);
    let commandline = format!(
        " {:?} -fast -noborder -an -sn -fast -x 2560 -y 1440 -loop 0 -loglevel quiet",
        absolute_file.as_path().to_str().unwrap()
    );

    unsafe {
        CreateProcessW(
            String::from(absolute_exe.as_path().to_str().unwrap()),
            commandline,
            &SECURITY_ATTRIBUTES {
                ..Default::default()
            },
            &SECURITY_ATTRIBUTES {
                ..Default::default()
            },
            BOOL::from(false),
            NORMAL_PRIORITY_CLASS,
            ::core::mem::zeroed(),
            PWSTR {
                ..Default::default()
            },
            &si,
            &mut pi,
        )
    };

    Ok(true)
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, _lparam: LPARAM) -> BOOL {
    let h_defview = FindWindowExW(
        hwnd,
        HWND {
            ..Default::default()
        },
        "SHELLDLL_DefView",
        PWSTR {
            ..Default::default()
        },
    );

    if !h_defview.is_invalid() {
        let h_workerw = FindWindowExW(
            HWND {
                ..Default::default()
            },
            hwnd,
            "WorkerW",
            PWSTR {
                ..Default::default()
            },
        );
        ShowWindow(h_workerw, SW_HIDE);
        return BOOL(0);
    }
    return BOOL(1);
}

fn main() {
    let args = env::args().nth(1);
    if let Some(arg) = args {
        let is_running = run_window(arg).unwrap();
        if is_running {
            unsafe {
                Sleep(300);

                let h_ffplay = FindWindowW(
                    "SDL_app",
                    PWSTR {
                        ..Default::default()
                    },
                );

                let h_program = FindWindowW(
                    "Progman",
                    PWSTR {
                        ..Default::default()
                    },
                );

                SendMessageTimeoutW(
                    h_program,
                    0x52C,
                    WPARAM {
                        ..Default::default()
                    },
                    LPARAM {
                        ..Default::default()
                    },
                    SMTO_NORMAL,
                    100,
                    &mut 0,
                );

                SetParent(h_ffplay, h_program);

                EnumWindows(
                    Some(enum_windows_proc),
                    LPARAM {
                        ..Default::default()
                    },
                );
            }
        }
    }
}
