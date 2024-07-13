use std::{
    path::Path,
    process::{Command, Output},
};

use ffmpeg_sidecar::{
    command::FfmpegCommand,
    ffprobe::{self, ffprobe_path},
};

pub fn init() {
    ffmpeg_sidecar::download::auto_download().unwrap()
}

pub fn dimensions_and_duration(path: &str) -> (u16, u16, u64) {
    let mut width: u16 = 0;
    let mut height: u16 = 0;
    let mut duration: u64 = 0;

    let output_result = Command::new(ffprobe_path())
        .args([
            "-v",
            "error",
            "-select_streams",
            "v",
            "-show_entries",
            "stream=width,height",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1",
        ])
        .arg(path)
        .output();

    match output_result {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(raw_output) => {
                let lines = raw_output.split("\n");
                for line in lines {
                    let key_value: Vec<&str> = line.split("=").collect();
                    if key_value.len() == 2 {
                        let key = key_value[0];
                        let value = key_value[1];

                        if key == "width" {
                            width = value.trim().parse().expect("Could not parse width");
                        } else if key == "height" {
                            height = value.trim().parse().expect("Could not parse height")
                        } else if key == "duration" {
                            let duration_float = value
                                .trim()
                                .parse::<f64>()
                                .expect("Could not parse duration");
                            duration = (duration_float * 1000.00) as u64
                        }
                    }
                }
            }
            Err(err) => eprintln!("Colud not read output: {}", err),
        },
        Err(err) => eprintln!(
            "Could not run ffprobe command to get dimensions and duration: {}",
            err
        ),
    }
    return (width, height, duration);
}
