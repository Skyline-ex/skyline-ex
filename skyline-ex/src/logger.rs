use std::cell::UnsafeCell;
use std::io::BufWriter;
use std::path::Path;
use std::sync::mpsc;
use std::time::SystemTime;
use std::{collections::VecDeque, io::Write};

use skyline::once_cell::sync::Lazy;

extern "C" {
    #[link_name = "_ZN2nn6socket10InitializeEPvmmi"]
    fn nn_socket_initialize(
        pool: *mut u8,
        pool_size: usize,
        alloc_pool_size: usize,
        concurrency_limit: i32,
    ) -> u32;

    #[link_name = "_ZN2nn4time10InitializeEv"]
    fn nn_time_initialize();
}

#[skyline::shim(replace = nn_socket_initialize)]
pub fn socket_init_shim(
    pool: *mut u8,
    pool_size: usize,
    alloc_pool_size: usize,
    concurrency_limit: i32,
) -> u32 {
    original(pool, pool_size, alloc_pool_size, concurrency_limit)
}

#[skyline::shim(replace = nn_time_initialize)]
pub fn time_init_shim() {
    original()
}

/// Since we can't rely on most time based libraries, this is a seconds -> date/time string based on the `chrono` crates implementation
fn format_time_string(seconds: u64) -> String {
    let leapyear = |year| -> bool { year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) };

    static YEAR_TABLE: [[u64; 12]; 2] = [
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    ];

    let mut year = 1970;

    let seconds_in_day = seconds % 86400;
    let mut day_number = seconds / 86400;

    let sec = seconds_in_day % 60;
    let min = (seconds_in_day % 3600) / 60;
    let hours = seconds_in_day / 3600;
    loop {
        let year_length = if leapyear(year) { 366 } else { 365 };

        if day_number >= year_length {
            day_number -= year_length;
            year += 1;
        } else {
            break;
        }
    }
    let mut month = 0;
    while day_number >= YEAR_TABLE[if leapyear(year) { 1 } else { 0 }][month] {
        day_number -= YEAR_TABLE[if leapyear(year) { 1 } else { 0 }][month];
        month += 1;
    }

    format!(
        "{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
        year,
        month + 1,
        day_number + 1,
        hours,
        min,
        sec
    )
}

const POOL_SIZE: usize = 0x600000;
const POOL_ALIGN: usize = 0x4000;
const ALLOC_POOL_SIZE: usize = 0x20000;
const CONCURRENCY_LIMIT: i32 = 14;
const PORT: u16 = 6969;

pub enum LoggerCommand {
    Print(String),
    Flush,
    Terminate,
}

pub struct LoggerInfo {
    commands: mpsc::Sender<LoggerCommand>,
    handle: UnsafeCell<Option<std::thread::JoinHandle<()>>>,
}

impl LoggerInfo {
    pub fn print<S: AsRef<str>>(&self, message: S) -> Result<(), mpsc::SendError<LoggerCommand>> {
        self.commands
            .send(LoggerCommand::Print(message.as_ref().to_string()))
    }

    pub fn flush(&self) -> Result<(), mpsc::SendError<LoggerCommand>> {
        self.commands.send(LoggerCommand::Flush)
    }

    pub fn terminate(&self) {
        if self.commands.send(LoggerCommand::Terminate).is_err() {
            return;
        }

        if let Some(handle) = unsafe { (*self.handle.get()).take() } {
            let _ = handle.join();
        }
    }
}

unsafe impl Sync for LoggerInfo {}
unsafe impl Send for LoggerInfo {}

pub static LOGGER: Lazy<LoggerInfo> = Lazy::new(|| {
    #[cfg(feature = "tcp-logger")]
    unsafe {
        socket_init_shim::install();

        let layout = std::alloc::Layout::from_size_align(POOL_SIZE, POOL_ALIGN).unwrap();
        let raw_buffer = std::alloc::alloc(layout);
        let _ = socket_init_shim(
            raw_buffer,
            layout.size(),
            ALLOC_POOL_SIZE,
            CONCURRENCY_LIMIT,
        );
    }

    let mut file = if cfg!(feature = "sd-logger") {
        'init_file: {
            time_init_shim::install();
            time_init_shim();

            let seconds = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();

            if std::fs::create_dir_all("sd:/skyline-ex/logs").is_err() {
                break 'init_file None;
            }

            let path = Path::new("sd:/skyline-ex/logs")
                .join(format!("{}.log", format_time_string(seconds.as_secs())));

            std::fs::File::create(path).map(BufWriter::new).ok()
        }
    } else {
        None
    };
    let (tx, rx) = mpsc::channel();

    let handle = std::thread::spawn(move || {
        #[cfg(feature = "tcp-logger")]
            let Ok(server) = std::net::TcpListener::bind("0.0.0.0:6969") else {
                return;
            };

        // We need to spawn a second thread for maintaining connections `TcpListener::set_nonblocking` does not
        // work on current STD
        #[cfg(feature = "tcp-logger")]
        let (connection_tx, connection_rx) = mpsc::channel();

        // TODO: Maybe cleanup/track this handle? The logger will only get terminated on
        // application close
        #[cfg(feature = "tcp-logger")]
        {
            let _ = std::thread::spawn(move || loop {
                if let Ok((stream, _)) = server.accept() {
                    let _ = stream.set_nonblocking(false);
                    connection_tx.send(stream).unwrap();
                }
            });
        }

        // Manage a vec of connections and messages, the messages we need as double-sided
        // but the vec is just to maintain a list of only active connections
        #[cfg(feature = "tcp-logger")]
        let mut connections = Vec::new();
        #[cfg(feature = "tcp-logger")]
        let mut tcp_messages = VecDeque::new();

        let mut messages = VecDeque::new();
        loop {
            // Sleep for a minimum of 1 millisecond between each message sent, just to allow other threads to work
            std::thread::sleep(std::time::Duration::from_millis(1));

            // Check for an incoming connection
            #[cfg(feature = "tcp-logger")]
            if let Ok(stream) = connection_rx.try_recv() {
                connections.push(stream);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }

            // Check for an incoming command
            let (should_flush, should_terminate) = if let Ok(command) = rx.try_recv() {
                match command {
                    LoggerCommand::Print(message) => {
                        #[cfg(feature = "tcp-logger")]
                        {
                            tcp_messages.push_back(message.clone());
                        }
                        messages.push_back(message);
                        (false, false)
                    }
                    LoggerCommand::Flush => (true, false),
                    LoggerCommand::Terminate => (true, true),
                }
            } else {
                (false, false)
            };

            // If we don't have any active connections then don't log anything, we want to preserve our messages
            // if we can
            #[cfg(all(
                feature = "tcp-logger",
                not(feature = "sd-logger"),
                not(feature = "kernel-logger")
            ))]
            if connections.is_empty() {
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }

            // Determine how many messages we should send per loop
            let end = if should_flush { messages.len() } else { 1 };

            for _ in 0..end {
                // If we run out of messages then jump out
                let Some(message) = messages.pop_front() else {
                    break;
                };

                
                if let Some(file) = file.as_mut() {
                    let _ = file.write(message.as_bytes());
                    let _ = file.flush();
                }
                
                #[cfg(feature = "kernel-logger")]
                {
                    let _ = skyline::nx::output_debug_string(&message);
                }
            }

            // Update our list of connections while we attempt to send out a message
            #[cfg(feature = "tcp-logger")]
            if !connections.is_empty() {
                let end = if should_flush { tcp_messages.len() } else { 1 };

                for _ in 0..end {
                    let Some(message) = tcp_messages.pop_front() else {
                        break;
                    };

                    connections = connections
                        .into_iter()
                        .filter_map(|mut stream| match stream.write(message.as_bytes()) {
                            Ok(_) => Some(stream),
                            Err(_) => None,
                        })
                        .collect();
                }
            }

            // Exit the thread if we should terminate
            if should_terminate {
                break;
            }

            // If we flushed then sleep for a little bit longer, so that in case it was a lot the application
            // can catch up
            if should_flush {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    });
    LoggerInfo {
        commands: tx,
        handle: UnsafeCell::new(Some(handle)),
    }
});

pub fn initialize() {
    Lazy::force(&LOGGER);
}