// Traverse all files in the specified directory under the Linux environment,
// determine whether to keep them based on the file modification time,
// and delete the files that do not need to be kept. Only keep the latest copies
// from one day ago, one week ago, one month ago, one year ago, and two years ago.

use std::fs;
use chrono::{DateTime, Duration, Local};
use clap::Parser;

#[derive(Parser, PartialEq, Debug)]
#[clap(name = "autoclear-backup", version = "1.0", author = "Hude", about = "auto clear old backup files")]
struct Opt {
    /// file prefix to filter files to be cleared
    /// If not specified, all files in the directory will be cleared.
    #[arg(short, long)]
    prefix: Option<String>,

    /// directory to be cleared
    /// If not specified, the current directory will be used.
    #[arg(index = 1)]
    directory: Option<String>,

    /// test mode, only print files to be removed, but do not actually remove them.
    #[arg(short, long, default_value_t = false)]
    test: bool,
}

fn main() {
    let cli = Opt::parse();

    // 如果directory不是以斜杠结尾，则需要加上斜杠
    let dir = if let Some(directory) = cli.directory {
        if directory.ends_with('/') {
            directory
        } else {
            format!("{}/", directory)
        }
    } else {
        "./".to_string()
    };

    clear_old_files(dir, &cli.prefix, cli.test);
}


// clear_old_files 函数用于清理指定目录下旧文件
// 参数：
// - directory: 要清理的文件夹路径
// - prefix: 文件名前缀，用于筛选需要清理的文件
// - test: 是否为测试模式，如果为 true，则只打印将要删除的文件，而不实际删除
// 
// 该函数会遍历指定目录下所有文件，根据文件修改时间判断是否需要保留，并删除不需要保留的文件。
// 保留的文件的修改时间会根据以下规则判断：
// - 前一天
// - 一周前
// - 一个月前
// - 一年前
// - 两年前
// 
fn clear_old_files(directory: String, prefix: &Option<String>, test: bool) {
    // get directory entries
    let entries = match fs::read_dir(&directory) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("无法读取目录: {}", e);
            return;
        }
    };

    // get current time
    let now = Local::now();

    // define keep dates:w
    let keep_dates = [
        now - Duration::days(0),    // 最新的
        now - Duration::days(1),    // 前一天
        now - Duration::weeks(1),   // 一周前
        now - Duration::weeks(4),   // 一个月前
        now - Duration::weeks(52),  // 一年前
        now - Duration::weeks(104), // 两年前
    ];

    // file list to be kept
    let mut keep_files = Vec::new();

    let mut fadd = |filename: &str| {
        let path = format!("{}{}", directory, filename);

        if let Ok(metadata) = fs::metadata(&path) {
            if let Ok(modified) = metadata.modified() {
                let modified_time = DateTime::<Local>::from(modified);
                keep_files.push((modified_time, path, true));
            }
        }
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                if let Some(prefix) = prefix {
                    if file_name.starts_with(prefix) {
                        fadd(file_name);
                    }
                } else {
                    fadd(file_name);
                }
            }
        }
    }

    // sort files by modified time
    keep_files.sort_by(|a, b| b.0.cmp(&a.0));

    // mark files to be removed
    if let Some(prefix) = prefix {
        println!("clearing files with prefix: '{}'", prefix);

        for keep_date in &keep_dates {
            for item in &mut keep_files {
                let (modified_time, ref path, ref mut clean) = item;

                if *modified_time < *keep_date {
                    if !test {
                        println!("keeping file: {}", path);
                    }

                    *clean = false;
                    break;
                }
            }
        }
    } else {
        println!("clearing all files in directory");
    }

    // remove files that are not marked as keep
    for (_, path, clean) in keep_files {
        if clean {
            if test {
                println!("remove file: {}", path);
            } else {
                if let Err(e) = fs::remove_file(&path) {
                    eprintln!("cannot remove file '{}': {}", path, e);
                }
            }
        }
    }
}
