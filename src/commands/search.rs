use serde::Serialize;
use std::fs;

/// Output when the search is still in progress.
#[derive(Serialize)]
struct SearchOutput {
    range: [f64; 2],
    segments: Vec<Segment>,
}

/// A single segment within the current range.
#[derive(Serialize)]
struct Segment {
    id: String,
    range: [f64; 2],
    size_bytes: usize,
    preview: String,
}

/// Output when the search has converged.
#[derive(Serialize)]
struct DoneOutput {
    done: bool,
    anchor: String,
}

/// Run `ae search` — sliding bisection to narrow a target scope.
pub fn run(
    file: &str,
    range: Option<&str>,
    termination_bytes: Option<usize>,
    preview_bytes: Option<usize>,
    overlap: Option<f64>,
) -> i32 {
    let termination_bytes = termination_bytes.unwrap_or(2000);
    let preview_bytes = preview_bytes.unwrap_or(256);
    let overlap_ratio = overlap.unwrap_or(0.1);

    // Parse range
    let (range_start, range_end) = match range {
        Some(r) => parse_range(r).unwrap_or_else(|| {
            eprintln!("error: invalid range format, expected start:end (e.g. 0.3:0.7)");
            std::process::exit(1);
        }),
        None => (0.0, 1.0),
    };

    // Read file
    let data = match fs::read(file) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: failed to read file: {e}");
            return 1;
        }
    };

    let file_size = data.len();
    if file_size == 0 {
        eprintln!("error: file is empty");
        return 1;
    }

    // Compute byte offsets for the range
    let start = (range_start * file_size as f64) as usize;
    let end = (range_end * file_size as f64) as usize;
    let range_size = end - start;

    // Termination check
    if range_size <= termination_bytes || range_size <= 200 {
        // Extract anchor (UTF-8 safe)
        let anchor = extract_utf8_safe(&data, start, end);
        let output = DoneOutput {
            done: true,
            anchor,
        };
        println!("{}", serde_json::to_string(&output).unwrap());
        return 0;
    }

    // Compute segments with overlap
    let overlap_bytes = (overlap_ratio * range_size as f64) as usize;

    // Segment A: [start, start + 0.4 * size]
    let a_end = start + (0.4 * range_size as f64) as usize;
    let a_start = start;

    // Segment B: [start + 0.3 * size, start + 0.7 * size]
    let b_start = start + (0.3 * range_size as f64) as usize;
    let b_end = start + (0.7 * range_size as f64) as usize;

    // Segment C: [start + 0.6 * size, end]
    let c_start = start + (0.6 * range_size as f64) as usize;
    let c_end = end;

    // Apply overlap: expand each segment by overlap_bytes
    let a_start = a_start.saturating_sub(overlap_bytes);
    let a_end = a_end.min(end).saturating_add(overlap_bytes).min(end);
    let b_start = b_start.saturating_sub(overlap_bytes);
    let b_end = b_end.min(end).saturating_add(overlap_bytes).min(end);
    let c_start = c_start.saturating_sub(overlap_bytes);
    let c_end = c_end.min(end).saturating_add(overlap_bytes).min(end);

    // Clamp to file bounds
    let a_start = a_start.min(file_size);
    let a_end = a_end.min(file_size);
    let b_start = b_start.min(file_size);
    let b_end = b_end.min(file_size);
    let c_start = c_start.min(file_size);
    let c_end = c_end.min(file_size);

    // Compute relative ranges (as fractions of file size)
    let to_frac = |offset: usize| offset as f64 / file_size as f64;

    let segments = vec![
        Segment {
            id: "A".to_string(),
            range: [to_frac(a_start), to_frac(a_end)],
            size_bytes: a_end - a_start,
            preview: make_preview(&data, a_start, a_end, preview_bytes),
        },
        Segment {
            id: "B".to_string(),
            range: [to_frac(b_start), to_frac(b_end)],
            size_bytes: b_end - b_start,
            preview: make_preview(&data, b_start, b_end, preview_bytes),
        },
        Segment {
            id: "C".to_string(),
            range: [to_frac(c_start), to_frac(c_end)],
            size_bytes: c_end - c_start,
            preview: make_preview(&data, c_start, c_end, preview_bytes),
        },
    ];

    let output = SearchOutput {
        range: [range_start, range_end],
        segments,
    };

    println!("{}", serde_json::to_string(&output).unwrap());
    0
}

/// Parse range string "start:end" into (f64, f64).
fn parse_range(s: &str) -> Option<(f64, f64)> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let start: f64 = parts[0].trim().parse().ok()?;
    let end: f64 = parts[1].trim().parse().ok()?;
    if start < 0.0 || end > 1.0 || start >= end {
        return None;
    }
    Some((start, end))
}

/// Extract a UTF-8 safe string from the byte slice.
fn extract_utf8_safe(data: &[u8], start: usize, end: usize) -> String {
    let slice = &data[start..end];
    String::from_utf8_lossy(slice).into_owned()
}

/// Create a preview by taking the first `max_bytes` bytes, ensuring UTF-8 safety.
fn make_preview(data: &[u8], start: usize, end: usize, max_bytes: usize) -> String {
    let slice = &data[start..end];
    let preview = if slice.len() > max_bytes {
        &slice[..max_bytes]
    } else {
        slice
    };
    String::from_utf8_lossy(preview).into_owned()
}
