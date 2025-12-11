
use std::path::{Path, PathBuf};
use crate::filter::FilterChain;

/// Codec vidéo supportés (facultatif mais pratique)
#[derive(Debug, Clone, Copy)]
pub enum VideoCodec {
    H264,
    H265,
    Nvenc,
}

impl VideoCodec {
    pub fn as_str(self) -> &'static str {
        match self {
            VideoCodec::H264 => "libx264",
            VideoCodec::H265 => "libx265",
            VideoCodec::Nvenc => "h264_nvenc",
        }
    }
}

/// Preset d’encodage (x264/h264_nvenc, etc.)
#[derive(Debug, Clone, Copy)]
pub enum Preset {
    Ultrafast,
    Superfast,
    Veryfast,
    Faster,
    Fast,
    Medium,
    Slow,
    Slower,
    Veryslow,
}

impl Preset {
    pub fn as_str(self) -> &'static str {
        match self {
            Preset::Ultrafast => "ultrafast",
            Preset::Superfast => "superfast",
            Preset::Veryfast => "veryfast",
            Preset::Faster => "faster",
            Preset::Fast => "fast",
            Preset::Medium => "medium",
            Preset::Slow => "slow",
            Preset::Slower => "slower",
            Preset::Veryslow => "veryslow",
        }
    }
}

#[derive(Debug)]
pub struct Job {
    pub input_video: String,
    pub input_images: Option<Vec<String>>,        // logos, watermarks…
    pub input_audio: Option<String>,      // musique / voix off

    pub filter: FilterChain,                // le graphe complet

    pub codec: VideoCodec,
    pub preset: Preset,
    pub crf: u8,

    pub output: String,
}

pub fn job_to_args(job: &Job) -> Vec<String> {
    let mut args = Vec::new();

    args.push("-y".into());

    args.push("-i".into());
    args.push(job.input_video.clone());

    if let Some(imgs) = &job.input_images {
        for img in imgs {
            args.push("-i".into());
            args.push(img.clone());
        }
    }

    if let Some(a) = &job.input_audio {
        args.push("-i".into());
        args.push(a.clone());
    }

    let filter_str = job.filter.clone().build();
    if !filter_str.is_empty() {
        args.push("-vf".into());
        args.push(filter_str);
    }

    args.push("-c:v".into());
    args.push(job.codec.as_str().to_string());
    args.push("-preset".into());
    args.push(job.preset.as_str().to_string());
    args.push("-crf".into());
    args.push(job.crf.to_string());

    if job.input_audio.is_some() {
        args.push("-c:a".into());
        args.push("aac".into());
        args.push("-b:a".into());
        args.push("192k".into());
    } else {
        args.push("-c:a".into());
        args.push("copy".into());
    }

    // args.push("-movflags".into());
    // args.push("+faststart".into());

    args.push(job.output.clone());

    args
}