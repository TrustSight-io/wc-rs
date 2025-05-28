use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::thread;

#[derive(Debug, Default, Clone)]
struct WcCounts {
    lines: usize,
    words: usize,
    chars: usize,
}

impl WcCounts {
    #[inline(always)]
    fn add(&mut self, other: &WcCounts) {
        self.lines += other.lines;
        self.words += other.words;
        self.chars += other.chars;
    }
}

#[derive(Debug)]
struct Config {
    show_lines: bool,
    show_words: bool,
    show_chars: bool,
    files: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            show_lines: true,
            show_words: true,
            show_chars: true,
            files: Vec::new(),
        }
    }
}

impl Config {
    fn from_args() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();
        let mut config = Config::default();
        let mut explicit_flags = false;

        let mut i = 1;
        while i < args.len() {
            let arg = &args[i];

            if arg.starts_with('-') && arg.len() > 1 {
                if !explicit_flags {
                    explicit_flags = true;
                    config.show_lines = false;
                    config.show_words = false;
                    config.show_chars = false;
                }

                for ch in arg.chars().skip(1) {
                    match ch {
                        'l' => config.show_lines = true,
                        'w' => config.show_words = true,
                        'c' => config.show_chars = true,
                        _ => return Err(format!("Unknown flag: -{}", ch)),
                    }
                }
            } else {
                config.files.push(arg.clone());
            }
            i += 1;
        }

        Ok(config)
    }
}

// 🚀 MAXIMUM BLAZING SPEED - Optimized for pure performance
#[inline(always)]
fn count_bytes_blazing_speed(data: &[u8]) -> WcCounts {
    let mut lines = 0usize;
    let mut words = 0usize;
    let chars = data.len();
    let mut in_word = false;

    // Process data in the most cache-friendly way possible
    let mut i = 0;
    let len = data.len();

    // Unrolled loop for maximum performance
    while i + 7 < len {
        let b0 = unsafe { *data.get_unchecked(i) };
        let b1 = unsafe { *data.get_unchecked(i + 1) };
        let b2 = unsafe { *data.get_unchecked(i + 2) };
        let b3 = unsafe { *data.get_unchecked(i + 3) };
        let b4 = unsafe { *data.get_unchecked(i + 4) };
        let b5 = unsafe { *data.get_unchecked(i + 5) };
        let b6 = unsafe { *data.get_unchecked(i + 6) };
        let b7 = unsafe { *data.get_unchecked(i + 7) };

        // Count newlines with branchless arithmetic
        lines += (b0 == b'\n') as usize;
        lines += (b1 == b'\n') as usize;
        lines += (b2 == b'\n') as usize;
        lines += (b3 == b'\n') as usize;
        lines += (b4 == b'\n') as usize;
        lines += (b5 == b'\n') as usize;
        lines += (b6 == b'\n') as usize;
        lines += (b7 == b'\n') as usize;

        // Ultra-fast whitespace detection
        let ws0 = b0 <= b' ';
        let ws1 = b1 <= b' ';
        let ws2 = b2 <= b' ';
        let ws3 = b3 <= b' ';
        let ws4 = b4 <= b' ';
        let ws5 = b5 <= b' ';
        let ws6 = b6 <= b' ';
        let ws7 = b7 <= b' ';

        // Word counting with minimal branching
        if !ws0 && !in_word {
            words += 1;
            in_word = true;
        } else if ws0 {
            in_word = false;
        }
        if !ws1 && !in_word {
            words += 1;
            in_word = true;
        } else if ws1 {
            in_word = false;
        }
        if !ws2 && !in_word {
            words += 1;
            in_word = true;
        } else if ws2 {
            in_word = false;
        }
        if !ws3 && !in_word {
            words += 1;
            in_word = true;
        } else if ws3 {
            in_word = false;
        }
        if !ws4 && !in_word {
            words += 1;
            in_word = true;
        } else if ws4 {
            in_word = false;
        }
        if !ws5 && !in_word {
            words += 1;
            in_word = true;
        } else if ws5 {
            in_word = false;
        }
        if !ws6 && !in_word {
            words += 1;
            in_word = true;
        } else if ws6 {
            in_word = false;
        }
        if !ws7 && !in_word {
            words += 1;
            in_word = true;
        } else if ws7 {
            in_word = false;
        }

        i += 8;
    }

    // Handle remaining bytes
    while i < len {
        let byte = unsafe { *data.get_unchecked(i) };

        if byte == b'\n' {
            lines += 1;
        }

        let is_whitespace = byte <= b' ';
        if !is_whitespace {
            if !in_word {
                words += 1;
                in_word = true;
            }
        } else {
            in_word = false;
        }

        i += 1;
    }

    WcCounts {
        lines,
        words,
        chars,
    }
}

// 🔥 MAXIMUM SPEED memory-mapped file reading
fn count_file_blazing_mmap(file_path: &str) -> Result<WcCounts, io::Error> {
    #[cfg(unix)]
    {
        use std::fs::File;
        use std::os::unix::io::AsRawFd;

        let file = File::open(file_path)?;
        let metadata = file.metadata()?;
        let file_size = metadata.len() as usize;

        if file_size == 0 {
            return Ok(WcCounts::default());
        }

        let fd = file.as_raw_fd();

        unsafe {
            let ptr = libc::mmap(
                std::ptr::null_mut(),
                file_size,
                libc::PROT_READ,
                libc::MAP_PRIVATE | libc::MAP_POPULATE,
                fd,
                0,
            );

            if ptr == libc::MAP_FAILED {
                return count_file_blazing_read(file_path);
            }

            // Prefetch for sequential access
            libc::madvise(ptr, file_size, libc::MADV_SEQUENTIAL);
            libc::madvise(ptr, file_size, libc::MADV_WILLNEED);

            let data = std::slice::from_raw_parts(ptr as *const u8, file_size);
            let counts = count_bytes_blazing_speed(data);

            libc::munmap(ptr, file_size);
            Ok(counts)
        }
    }

    #[cfg(not(unix))]
    count_file_blazing_read(file_path)
}

// 🚀 Optimized read fallback with massive buffers
fn count_file_blazing_read(file_path: &str) -> Result<WcCounts, io::Error> {
    let mut file = File::open(file_path)?;

    // Use 2MB buffer for maximum I/O efficiency
    const BUFFER_SIZE: usize = 2 * 1024 * 1024;
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut total_counts = WcCounts::default();
    let mut last_was_word_char = false;
    let mut first_chunk = true;

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let chunk = unsafe { buffer.get_unchecked(..bytes_read) };
        let mut chunk_counts = count_bytes_blazing_speed(chunk);

        // Handle word boundaries across buffer boundaries
        if !first_chunk && !chunk.is_empty() {
            let first_byte = unsafe { *chunk.get_unchecked(0) };
            let first_is_word_char = first_byte > b' ';

            // If the previous chunk ended with a word character and this chunk
            // starts with a word character, we counted one word too many
            if last_was_word_char && first_is_word_char {
                chunk_counts.words = chunk_counts.words.saturating_sub(1);
            }
        }

        // Remember the last character state for the next iteration
        if !chunk.is_empty() {
            let last_byte = unsafe { *chunk.get_unchecked(chunk.len() - 1) };
            last_was_word_char = last_byte > b' ';
        }

        total_counts.add(&chunk_counts);
        first_chunk = false;
    }

    Ok(total_counts)
}

// ⚡ PARALLEL processing optimized for maximum throughput
fn count_files_parallel_blazing(file_paths: &[String]) -> Vec<Result<WcCounts, io::Error>> {
    use std::sync::mpsc;

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(8)
        .min(file_paths.len())
        .max(1);

    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::with_capacity(num_threads);

    for (index, file_path) in file_paths.iter().enumerate() {
        let tx = tx.clone();
        let file_path = file_path.clone();

        let handle = thread::spawn(move || {
            let result = count_file_blazing_mmap(&file_path);
            tx.send((index, result)).unwrap();
        });

        handles.push(handle);
    }

    drop(tx);

    let mut results: Vec<Result<WcCounts, io::Error>> = (0..file_paths.len())
        .map(|_| Ok(WcCounts::default()))
        .collect();

    for (index, result) in rx {
        results[index] = result;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    results
}

// 🚀 Blazing stdin processing
fn count_stdin_blazing() -> Result<WcCounts, io::Error> {
    let mut stdin = io::stdin();
    let mut buffer = Vec::with_capacity(2 * 1024 * 1024);

    stdin.read_to_end(&mut buffer)?;
    Ok(count_bytes_blazing_speed(&buffer))
}

#[inline(always)]
fn format_output(counts: &WcCounts, config: &Config, filename: Option<&str>) -> String {
    let mut parts = Vec::with_capacity(3);

    if config.show_lines {
        parts.push(format!("{:8}", counts.lines));
    }
    if config.show_words {
        parts.push(format!("{:8}", counts.words));
    }
    if config.show_chars {
        parts.push(format!("{:8}", counts.chars));
    }

    let mut output = parts.join(" ");

    if let Some(name) = filename {
        output.push(' ');
        output.push_str(name);
    }

    output
}

fn main() {
    let config = match Config::from_args() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Usage: wc [-l] [-w] [-c] [file ...]");
            std::process::exit(1);
        }
    };

    if config.files.is_empty() {
        match count_stdin_blazing() {
            Ok(counts) => {
                println!("{}", format_output(&counts, &config, None));
            }
            Err(e) => {
                eprintln!("Error reading stdin: {}", e);
                std::process::exit(1);
            }
        }
    } else if config.files.len() == 1 {
        let file_path = &config.files[0];
        match count_file_blazing_mmap(file_path) {
            Ok(counts) => {
                println!("{}", format_output(&counts, &config, Some(file_path)));
            }
            Err(e) => {
                eprintln!("wc: {}: {}", file_path, e);
                std::process::exit(1);
            }
        }
    } else {
        let results = count_files_parallel_blazing(&config.files);
        let mut total_counts = WcCounts::default();

        for (i, result) in results.into_iter().enumerate() {
            let file_path = &config.files[i];
            match result {
                Ok(counts) => {
                    println!("{}", format_output(&counts, &config, Some(file_path)));
                    total_counts.add(&counts);
                }
                Err(e) => {
                    eprintln!("wc: {}: {}", file_path, e);
                    std::process::exit(1);
                }
            }
        }

        println!("{}", format_output(&total_counts, &config, Some("total")));
    }
}

// 🦀 Optimized libc bindings
#[cfg(unix)]
mod libc {
    pub const PROT_READ: i32 = 1;
    pub const MAP_PRIVATE: i32 = 2;
    pub const MAP_POPULATE: i32 = 0x8000;
    pub const MAP_FAILED: *mut std::ffi::c_void = !0 as *mut std::ffi::c_void;
    pub const MADV_SEQUENTIAL: i32 = 2;
    pub const MADV_WILLNEED: i32 = 3;

    extern "C" {
        pub fn mmap(
            addr: *mut std::ffi::c_void,
            len: usize,
            prot: i32,
            flags: i32,
            fd: i32,
            offset: i64,
        ) -> *mut std::ffi::c_void;

        pub fn munmap(addr: *mut std::ffi::c_void, len: usize) -> i32;

        pub fn madvise(addr: *mut std::ffi::c_void, len: usize, advice: i32) -> i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bytes_blazing_speed() {
        let text = b"Hello world\nThis is a test\n";
        let counts = count_bytes_blazing_speed(text);

        assert_eq!(counts.lines, 2);
        assert_eq!(counts.words, 6);
        assert_eq!(counts.chars, 27);
    }

    #[test]
    fn test_empty_bytes() {
        let counts = count_bytes_blazing_speed(&[]);
        assert_eq!(counts.lines, 0);
        assert_eq!(counts.words, 0);
        assert_eq!(counts.chars, 0);
    }

    #[test]
    fn test_chunked_processing() {
        let large_text = "word ".repeat(1000);
        let counts = count_bytes_blazing_speed(large_text.as_bytes());
        assert_eq!(counts.words, 1000);
    }
}
