use std::path::PathBuf;


#[derive(Debug, Default,Clone)]
pub struct FilterChain {
    parts: Vec<String>,      
}

impl FilterChain {
    pub fn new() -> Self { Self::default() }

    pub fn raw<S: Into<String>>(mut self, filter: S) {
        self.parts.push(filter.into());
    }

    pub fn scale(mut self, w: u32, h: u32, keep_ratio: bool) {
        let mut s = format!("scale={}:{}", w, h);
        if keep_ratio {
            s.push_str(":force_original_aspect_ratio=increase");
        }
        self.parts.push(s);
    }

    pub fn crop_center(mut self, w: u32, h: u32) {
        self.parts.push(format!("crop={}:{}", w, h));
    }

    pub fn drawtext(
        mut self,
        font_path: String,
        text: &str,
        size: u32,
        color: &str,
        x: &str,
        y: &str,
        start: Option<f64>,
        end: Option<f64>,
    ) -> Self {

        let mut filter = format!(
            "drawtext=fontfile={}:text='{}':fontsize={}:fontcolor={}:x={}:y={}",
            font_path,
            text,
            size,
            color,
            x,
            y
        );

        if let (Some(s), Some(e)) = (start, end) {
            filter.push_str(&format!(":enable='between(t,{},{})'", s, e));
        }

        self.parts.push(filter);
        self
    }

    pub fn build(self) -> String {
        self.parts.join(",")
    }
}