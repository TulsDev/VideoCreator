
mod filter;
mod job;

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use filter::FilterChain;
use job::{Job,job_to_args,Preset,VideoCodec};

fn run_ffmpeg(args: &[String]) -> anyhow::Result<()> {
    
    println!("ffmpeg {}", args.join(" "));

    let mut child = Command::new("ffmpeg")
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()?; // std::process::Child

    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| anyhow::anyhow!("Pas de stderr à lire"))?;
    let reader = BufReader::new(stderr);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("frame=") || line.contains("time=") {
            println!("[ffmpeg] {}", line);
        }
    }

    let status = child.wait()?; // std::process::ExitStatus
    if !status.success() {
        let code = status.code().map(|c| c.to_string()).unwrap_or_else(|| "<unknown>".into());
        anyhow::bail!("ffmpeg a échoué (code {})", code);
    }
    Ok(())
}

fn main() {

    let assets_dir = "assets";
    let font_path = format!("{}/MozillaText-VariableFont_wght.ttf", assets_dir);

    let filter = FilterChain::new()
        .drawtext(font_path, "Hello world", 48, "white", "10", "10", None, None);

    let job = Job {
        input_video:  format!("{}/video.mp4", assets_dir),
        input_images: None,
        input_audio: None,
        filter,
        codec: VideoCodec::H264,
        preset: Preset::Fast,
        crf: 23,
        output: String::from("out/final.mp4"),
    };

    let args = job_to_args(&job);

    if let Err(e) = run_ffmpeg(&args) {
        eprintln!("Erreur: {}", e);
        std::process::exit(1);
    }

}
