use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
}

pub fn find_apps() -> Vec<AppEntry> {
    let mut apps = Vec::new();
    let home = std::env::var("HOME").unwrap_or_default();
    let paths = vec![
        "/usr/share/applications".to_string(),
        "/usr/local/share/applications".to_string(),
        format!("{}/.local/share/applications", home),
    ];

    for path in paths {
        let p = Path::new(&path);
        if !p.exists() {
            continue;
        }

        for entry in WalkDir::new(p).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("desktop") {
                if let Some(app) = parse_desktop_file(entry.path()) {
                    apps.push(app);
                }
            }
        }
    }
    
    // Deduplicate by name for simplicity
    apps.sort_by(|a, b| a.name.cmp(&b.name));
    apps.dedup_by(|a, b| a.name == b.name);
    
    apps
}

fn parse_desktop_file(path: &Path) -> Option<AppEntry> {
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    let mut name = None;
    let mut exec = None;
    let mut icon = None;
    let mut in_desktop_entry = false;
    let mut no_display = false;
    let mut is_application = false;

    for line in reader.lines().filter_map(|l| l.ok()) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if line == "[Desktop Entry]" {
            in_desktop_entry = true;
            continue;
        } else if line.starts_with('[') && line.ends_with(']') {
            in_desktop_entry = false;
            continue;
        }

        if in_desktop_entry {
            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "Name" => {
                        if name.is_none() { // Prefer the first Name entry (usually default locale)
                            name = Some(value.trim().to_string());
                        }
                    },
                    "Exec" => exec = Some(value.trim().to_string()),
                    "Icon" => icon = Some(value.trim().to_string()),
                    "NoDisplay" => no_display = value.trim().to_lowercase() == "true",
                    "Type" => is_application = value.trim() == "Application",
                    _ => {}
                }
            }
        }
    }

    if no_display || !is_application {
        return None;
    }

    match (name, exec) {
        (Some(n), Some(e)) => Some(AppEntry { 
            name: n, 
            exec: clean_exec(&e), 
            icon 
        }),
        _ => None,
    }
}

fn clean_exec(exec: &str) -> String {
    // Remove field codes like %f, %F, %u, %U, etc.
    exec.split_whitespace()
        .filter(|part| !part.starts_with('%'))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_desktop_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.desktop");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "[Desktop Entry]").unwrap();
        writeln!(file, "Name=Test App").unwrap();
        writeln!(file, "Exec=test-app --opt %u").unwrap();
        writeln!(file, "Type=Application").unwrap();

        let app = parse_desktop_file(&file_path).unwrap();
        assert_eq!(app.name, "Test App");
        assert_eq!(app.exec, "test-app --opt");
    }

    #[test]
    fn test_parse_no_display() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.desktop");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "[Desktop Entry]").unwrap();
        writeln!(file, "Name=Hidden").unwrap();
        writeln!(file, "Exec=hidden").unwrap();
        writeln!(file, "Type=Application").unwrap();
        writeln!(file, "NoDisplay=true").unwrap();

        let app = parse_desktop_file(&file_path);
        assert!(app.is_none());
    }
}
