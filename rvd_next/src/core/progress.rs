use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ProgressTracker {
    bars: Arc<Mutex<HashMap<String, ProgressBar>>>,
    multi_progress: MultiProgress,
}

impl ProgressTracker {
    pub fn new() -> Self {
        Self {
            bars: Arc::new(Mutex::new(HashMap::new())),
            multi_progress: MultiProgress::new(),
        }
    }

    pub fn create_bar(&self, id: &str, total: u64) -> Arc<ProgressBar> {
        let pb = self.multi_progress.add(ProgressBar::new(total));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                .expect("Failed to set progress style")
                .progress_chars("#>-"),
        );
        pb.set_message(id.to_string());

        let pb_arc = Arc::new(pb.clone());
        self.bars.lock().unwrap().insert(id.to_string(), pb);

        pb_arc
    }

    #[allow(dead_code)]
    pub fn update(&self, id: &str, current: u64) {
        if let Some(pb) = self.bars.lock().unwrap().get(id) {
            pb.set_position(current);
        }
    }

    pub fn finish(&self, id: &str, message: &str) {
        if let Some(pb) = self.bars.lock().unwrap().get(id) {
            pb.finish_with_message(message.to_string());
        }
    }

    pub fn finish_all(&self) {
        for (_, pb) in self.bars.lock().unwrap().iter() {
            pb.finish();
        }
    }
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}
