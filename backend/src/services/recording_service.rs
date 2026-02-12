use crate::errors::AppError;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

pub struct RecordingWriter {
    file: File,
    start_time: Instant,
}

impl RecordingWriter {
    pub fn new(recordings_dir: &str, session_id: &str, cols: u32, rows: u32) -> Result<Self, AppError> {
        fs::create_dir_all(recordings_dir)
            .map_err(|e| AppError::Internal(format!("Failed to create recordings dir: {}", e)))?;

        let path = PathBuf::from(recordings_dir).join(format!("{}.cast", session_id));
        let mut file = File::create(&path)
            .map_err(|e| AppError::Internal(format!("Failed to create recording file: {}", e)))?;

        let timestamp = chrono::Utc::now().timestamp();
        let header = serde_json::json!({
            "version": 2,
            "width": cols,
            "height": rows,
            "timestamp": timestamp,
            "env": { "TERM": "xterm-256color" }
        });

        writeln!(file, "{}", header)
            .map_err(|e| AppError::Internal(format!("Failed to write recording header: {}", e)))?;

        Ok(Self {
            file,
            start_time: Instant::now(),
        })
    }

    pub fn write_output(&mut self, data: &[u8]) -> Result<(), AppError> {
        self.write_event("o", data)
    }

    pub fn write_input(&mut self, data: &[u8]) -> Result<(), AppError> {
        self.write_event("i", data)
    }

    fn write_event(&mut self, event_type: &str, data: &[u8]) -> Result<(), AppError> {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let text = String::from_utf8_lossy(data);
        let line = serde_json::json!([elapsed, event_type, text.as_ref()]);

        writeln!(self.file, "{}", line)
            .map_err(|e| AppError::Internal(format!("Failed to write recording event: {}", e)))?;

        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), AppError> {
        self.file
            .flush()
            .map_err(|e| AppError::Internal(format!("Failed to flush recording: {}", e)))?;
        Ok(())
    }
}

pub fn recording_path(recordings_dir: &str, session_id: &str) -> String {
    format!("{}/{}.cast", recordings_dir, session_id)
}
