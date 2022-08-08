use image_compressor::Factor;
use image_compressor::FolderCompressor;
use std::cmp::max;
use std::env;
use std::path::PathBuf;
use std::sync::mpsc;

fn main() {
    let args: Vec<String> = env::args().collect();
    let origin = PathBuf::from(&args[1]); // original directory path
    let dest = PathBuf::from(&args[2]); // destination directory path
    let thread_count = 16; // number of threads
    let (tx, tr) = mpsc::channel(); // Sender and Receiver. for more info, check mpsc and message passing.

    let mut comp = FolderCompressor::new(origin, dest);
    comp.set_cal_func(|width, height, file_size| {
        let max_length: f32 = 1600.0;
        let max_size: f32 = 0.5 * 1024.0 * 1024.0;
        let max = max(width, height) as f32;
        let factor: f32 = if max >= max_length {
            max_length / max
        } else {
            1.0
        };
        let cur_resize_file_size = (file_size as f32) * factor * factor;
        let quality: f32 = if cur_resize_file_size >= max_size {
            max_size / cur_resize_file_size * 100.0
        } else {
            100.0
        };
        Factor::new(quality, factor)
    });
    comp.set_thread_count(thread_count);
    comp.set_sender(tx);

    match comp.compress() {
        Ok(_) => {}
        Err(e) => println!("Cannot compress the folder!: {}", e),
    }
}
