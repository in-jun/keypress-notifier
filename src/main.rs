use std::fs::File;
use std::io::{self, Read, Write};
use std::thread;

#[repr(C)]
pub struct Timeval {
    tv_sec: i64,
    tv_usec: i64,
}

#[repr(C)]
struct InputEvent {
    time: Timeval,
    type_: u16,
    code: u16,
    value: i32,
}

fn process_event(ev: &InputEvent) {
    if ev.type_ == 1 && (ev.value == 0 || ev.value == 1) {
        print!("\x07");
        let _ = io::stdout().flush();
    }
}

fn read_and_process_events(fd: &mut File) {
    let mut ev: InputEvent = unsafe { std::mem::zeroed() };
    while fd
        .read_exact(unsafe {
            std::slice::from_raw_parts_mut(
                (&mut ev) as *mut _ as *mut u8,
                std::mem::size_of::<InputEvent>(),
            )
        })
        .is_ok()
    {
        process_event(&ev);
    }
}

fn main() -> io::Result<()> {
    let mut fds: Vec<File> = Vec::new();

    let mut count: i32 = 0;
    loop {
        let device: String = format!("/dev/input/event{}", count);
        match File::open(&device) {
            Ok(fd) => {
                println!("Opened {} for reading.", device);
                fds.push(fd);
                count += 1;
            }
            Err(_) => break,
        }
    }

    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
    for fd in fds.iter() {
        let mut fd_clone: File = fd.try_clone()?;
        threads.push(thread::spawn(move || {
            read_and_process_events(&mut fd_clone);
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    Ok(())
}
