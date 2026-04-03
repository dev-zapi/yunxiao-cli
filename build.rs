use std::process::Command;

fn main() {
    // 获取 git commit hash
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|output| {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                "unknown".to_string()
            }
        })
        .unwrap_or_else(|_| "unknown".to_string());

    // 获取 build 时间
    let build_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 设置环境变量供代码使用
    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", git_hash);
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);

    // 如果 .git 目录变化则重新构建
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
}
