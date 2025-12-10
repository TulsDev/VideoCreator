use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

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

    let args = [
        s("-y"),
        
        s("-i"),
        s("assets/video.mp4"),

        // s("-aspect"),
        // s("9:16"),

        // s("-vf"),
        // s("scale=1080:1920"),

        s("-vf"),
        s("drawtext=fontfile=/assets/Mozilla_Text/MozillaText-VariableFont_wght/ttf:text='Hello World!':fontsize=48:fontcolor=white:x=0:y=0"),

        s("-vf"),
        s("scale=1080:1920:force_original_aspect_ratio=decrease,pad=1080:1920:(ow-iw)/2:(oh-ih)/2,setsar=1"),

        s("-c:v"),
        s("libx264"),
        
        s("-preset"),
        s("fast"),
        
        s("-crf"),
        s("23"),
        
        s("out/final.mp4"),
    ];

    if let Err(e) = run_ffmpeg(&args) {
        eprintln!("Erreur: {}", e);
        std::process::exit(1);
    }

}
