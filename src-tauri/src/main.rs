// Prevents an extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// A material-name record. Mirrors the JS record shape ({n, service?, series?, division?})
/// plus an optional timestamp so merges can prefer the newest entry.
#[derive(Serialize, Deserialize, Clone)]
struct Material {
    n: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    series: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    division: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    ts: Option<String>,
}

#[derive(Deserialize)]
struct RenameOp {
    path: String,
    new_name: String,
}

#[derive(Serialize)]
struct RenameResult {
    path: String,
    new_path: String,
    from: String,
    to: String,
    ok: bool,
    error: Option<String>,
}

// ===== Data folder (visible, user-friendly, survives app updates) =====
// %APPDATA%\파일명변경기\
//   ├─ materials.jsonl   (master DB, appended on every successful rename)
//   ├─ inbox\            (drop a shared file here to merge it in)
//   │   └─ _merged\      (processed files are moved here)
//   └─ exports\          ("내보내기" writes the shareable copy here)

fn data_dir() -> PathBuf {
    let base = std::env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir());
    base.join("파일명변경기")
}

fn ensure_dirs() -> PathBuf {
    let d = data_dir();
    let _ = fs::create_dir_all(d.join("inbox").join("_merged"));
    let _ = fs::create_dir_all(d.join("exports"));
    // 컴맹용 폴더 안내문 (없을 때만 생성)
    let guide = d.join("사용법.txt");
    if !guide.exists() {
        let _ = fs::write(&guide, GUIDE_TEXT);
    }
    d
}

const GUIDE_TEXT: &str = "\
[파일명 일괄 변경기 — 소재 폴더 안내]

이 폴더는 '소재명'(파일 이름에 들어가는 이름) 목록을 보관하는 곳입니다.
프로그램이 알아서 관리하니, 평소에는 신경 쓰지 않아도 됩니다.

- materials.jsonl : 내가 사용한 소재명이 한 줄에 하나씩 쌓이는 '주소록' 파일입니다.
- inbox\\         : 다른 사람이 보내준 소재명 파일(.jsonl 또는 .txt)을 여기에 넣어두면,
                    프로그램을 다시 켤 때 자동으로 내 목록에 합쳐집니다. (받은 파일 넣는 우체통)
- inbox\\_merged\\ : 합치기가 끝난 파일이 자동으로 이쪽으로 옮겨집니다. (중복 합침 방지용 — 그냥 두세요)
- exports\\        : 프로그램의 '내보내기' 버튼을 누르면, 남에게 보낼 공유용 복사본이 여기에 생깁니다.

[공유하는 법]
1) 보내기 : 프로그램에서 '내보내기' → exports 폴더에 생긴 파일을 카카오톡/메일로 전송하세요.
2) 받기   : 받은 파일을 inbox 폴더에 넣고 프로그램을 다시 켜세요.
            (또는 그 파일을 프로그램 창에 바로 끌어다 놓아도 합쳐집니다.)
";

/// Parse a materials file. Accepts JSON-object lines ({"n":"...",...}) and also
/// plain "one name per line" text, so a simple shared .txt works too.
fn read_materials_file(path: &Path) -> Vec<Material> {
    let mut out = Vec::new();
    if let Ok(content) = fs::read_to_string(path) {
        for raw in content.lines() {
            let line = raw.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with('{') {
                if let Ok(m) = serde_json::from_str::<Material>(line) {
                    if !m.n.trim().is_empty() {
                        out.push(m);
                    }
                }
            } else {
                out.push(Material {
                    n: line.to_string(),
                    service: None,
                    series: None,
                    division: None,
                    ts: None,
                });
            }
        }
    }
    out
}

/// Case-insensitive dedupe by `n`, last occurrence wins (newest appended).
/// Returns newest-first so the suggestion list shows recent names at the top.
fn dedupe(mats: Vec<Material>) -> Vec<Material> {
    use std::collections::HashMap;
    let mut map: HashMap<String, Material> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for m in mats {
        let k = m.n.to_lowercase();
        if !map.contains_key(&k) {
            order.push(k.clone());
        }
        map.insert(k, m);
    }
    order
        .into_iter()
        .rev()
        .filter_map(|k| map.get(&k).cloned())
        .collect()
}

fn append_materials(master: &Path, mats: &[Material]) {
    if mats.is_empty() {
        return;
    }
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(master) {
        for m in mats {
            if let Ok(line) = serde_json::to_string(m) {
                let _ = writeln!(f, "{}", line);
            }
        }
    }
}

/// Merge any *.jsonl / *.txt sitting in inbox\ into the master DB, then move
/// each processed file into inbox\_merged\ so it is never merged twice.
fn scan_inbox(dir: &Path) {
    let inbox = dir.join("inbox");
    let merged = inbox.join("_merged");
    let _ = fs::create_dir_all(&merged);
    let master = dir.join("materials.jsonl");
    if let Ok(entries) = fs::read_dir(&inbox) {
        for entry in entries.flatten() {
            let p = entry.path();
            if !p.is_file() {
                continue;
            }
            let ext = p
                .extension()
                .and_then(|x| x.to_str())
                .unwrap_or("")
                .to_lowercase();
            if ext != "jsonl" && ext != "txt" {
                continue;
            }
            let mats = read_materials_file(&p);
            append_materials(&master, &mats);
            if let Some(name) = p.file_name() {
                let mut dest = merged.join(name);
                let mut i = 2;
                while dest.exists() {
                    dest = merged.join(format!("{}_{}", i, name.to_string_lossy()));
                    i += 1;
                }
                let _ = fs::rename(&p, &dest);
            }
        }
    }
}

// ===== Commands =====

/// Rename files in place. Same-directory move only; refuses to overwrite an
/// existing file. No-op (reported ok) when the name is unchanged.
#[tauri::command]
fn rename_files(ops: Vec<RenameOp>) -> Vec<RenameResult> {
    ops.into_iter()
        .map(|op| {
            let src = PathBuf::from(&op.path);
            let from = src
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            let parent = src.parent().map(|p| p.to_path_buf()).unwrap_or_default();
            let dest = parent.join(&op.new_name);

            if from == op.new_name {
                return RenameResult {
                    path: op.path.clone(),
                    new_path: op.path,
                    from,
                    to: op.new_name,
                    ok: true,
                    error: None,
                };
            }
            if dest.exists() {
                return RenameResult {
                    path: op.path,
                    new_path: String::new(),
                    from,
                    to: op.new_name,
                    ok: false,
                    error: Some("같은 이름의 파일이 이미 있어요".into()),
                };
            }
            match fs::rename(&src, &dest) {
                Ok(_) => RenameResult {
                    path: op.path,
                    new_path: dest.to_string_lossy().to_string(),
                    from,
                    to: op.new_name,
                    ok: true,
                    error: None,
                },
                Err(e) => RenameResult {
                    path: op.path,
                    new_path: String::new(),
                    from,
                    to: op.new_name,
                    ok: false,
                    error: Some(e.to_string()),
                },
            }
        })
        .collect()
}

/// Read up to ~100 MB of a file's bytes (enough for image decode + header
/// parsers) and return them as a raw ArrayBuffer to the frontend. On any error
/// the result is empty bytes, so the frontend simply leaves resolution blank.
#[tauri::command]
fn read_file_bytes(path: String) -> tauri::ipc::Response {
    const CAP: u64 = 100 * 1024 * 1024;
    let bytes = (|| -> std::io::Result<Vec<u8>> {
        let f = fs::File::open(&path)?;
        let len = f.metadata().map(|m| m.len()).unwrap_or(0);
        let to_read = len.min(CAP);
        let mut buf = Vec::with_capacity(to_read as usize);
        f.take(to_read).read_to_end(&mut buf)?;
        Ok(buf)
    })()
    .unwrap_or_default();
    tauri::ipc::Response::new(bytes)
}

/// Merge inbox, then return the deduped material list (newest first).
#[tauri::command]
fn load_materials() -> Vec<Material> {
    let dir = ensure_dirs();
    scan_inbox(&dir);
    let master = dir.join("materials.jsonl");
    dedupe(read_materials_file(&master))
}

/// Append one material record to the master DB.
#[tauri::command]
fn add_material(rec: Material) -> Result<(), String> {
    let dir = ensure_dirs();
    let master = dir.join("materials.jsonl");
    append_materials(&master, std::slice::from_ref(&rec));
    Ok(())
}

/// Import an arbitrary .jsonl/.txt path (e.g. a file dropped onto the window),
/// merge it, and return the refreshed list.
#[tauri::command]
fn import_file(path: String) -> Vec<Material> {
    let dir = ensure_dirs();
    let master = dir.join("materials.jsonl");
    let mats = read_materials_file(Path::new(&path));
    append_materials(&master, &mats);
    dedupe(read_materials_file(&master))
}

/// Copy the master DB to exports\<filename> for sharing; return its full path.
#[tauri::command]
fn export_materials(filename: String) -> Result<String, String> {
    let dir = ensure_dirs();
    let master = dir.join("materials.jsonl");
    if !master.exists() {
        fs::write(&master, b"").map_err(|e| e.to_string())?;
    }
    let exports = dir.join("exports");
    let _ = fs::create_dir_all(&exports);
    let safe = filename.replace(['/', '\\'], "_");
    let dest = exports.join(safe);
    fs::copy(&master, &dest).map_err(|e| e.to_string())?;
    Ok(dest.to_string_lossy().to_string())
}

/// Open the data folder in the OS file explorer, with `path` selected/highlighted.
/// Used by "내보내기" so the user immediately sees the file they need to share.
#[tauri::command]
fn reveal_in_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(format!("/select,{}", path))
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        let parent = std::path::Path::new(&path)
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        std::process::Command::new("xdg-open")
            .arg(parent)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Open the data folder in the OS file explorer.
#[tauri::command]
fn open_data_folder() -> Result<(), String> {
    let dir = ensure_dirs();
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        std::process::Command::new("xdg-open")
            .arg(&dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Return a file's last-modified time as milliseconds since the Unix epoch.
#[tauri::command]
fn file_mtime(path: String) -> Option<f64> {
    let meta = fs::metadata(&path).ok()?;
    let mt = meta.modified().ok()?;
    let dur = mt.duration_since(std::time::UNIX_EPOCH).ok()?;
    Some(dur.as_millis() as f64)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            rename_files,
            read_file_bytes,
            load_materials,
            add_material,
            import_file,
            export_materials,
            reveal_in_folder,
            open_data_folder,
            file_mtime
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
