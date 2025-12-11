
mod filter;
mod job;

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use filter::FilterChain;
use job::{Job,job_to_args,Preset,VideoCodec};
use std::path::PathBuf;

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

fn s(s: &str) -> String {
    String::from(s)
}

fn main() {

    // let args = [
    //     s("-y"),
        
    //     s("-i"),
    //     s("assets/video.mp4"),

    //     s("-filter:v"),
    //     s("
    //     scale=1080:1920:force_original_aspect_ratio=increase,crop=1080:1920,
    //     drawtext=fontfile=/assets/Mozilla_Text/MozillaText-VariableFont_wght.ttf:text='Hello World!':fontsize=48:fontcolor=white:x=0:y=0,
    //     "),

    //     s("-c:v"),
    //     s("libx264"),
        
    //     s("-preset"),
    //     s("fast"),
        
    //     s("-crf"),
    //     s("23"),
        
    //     s("out/final.mp4"),
    // ];

    let assets_dir = "assets";
    let font_path = format!("{}/MozillaText-VariableFont_wght.ttf", assets_dir);

    let filter = FilterChain::new()
        .drawtext(font_path, "Hello world", 48, "white", "10", "10", None, None);

    let job = Job {
        input_video:  format!("{}/video.mp4", assets_dir),
        input_images: None, // vec![logo_path],
        input_audio: None, //Some(assets_dir + "music.mp3"), // <- vous pouvez mettre None
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
