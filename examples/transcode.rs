use std::io::{Read, Write};
use std::process::Command;

fn transcode_video(raw_video_data: &[u8]) -> Vec<u8> {
    // Use ffmpeg to transcode the raw video data
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", "-", "-c:v", "libx264", "-crf", "23", "-f", "mpegts", "-",
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = output.stdin.unwrap();
    let mut stdout = output.stdout.unwrap();

    stdin.write_all(raw_video_data).unwrap();
    stdin.flush().unwrap();
    stdin.shutdown().unwrap();

    let mut transcoded_video_data = vec![];
    stdout.read_to_end(&mut transcoded_video_data).unwrap();

    // Return the transcoded video data
    transcoded_video_data
}
