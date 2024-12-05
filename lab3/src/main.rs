use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use lab3::geometry_analyser::*;
use lab3::text_preprocessor::{preprocess, restore_dots};
use lab3::udpipe_api::*;

#[tokio::main]
async fn main() {
    let task =
        "
Периметр прямокутника 28 см, а відношення сторін 3:4. Знайдіть довжину описаного кола

        ";
    let task_refactored = preprocess(task);
    let tasks_processed = extract_significant_words(&process_text(&*task_refactored, "ukrainian").await.expect("REASON"));
    let tasks_with_dots = restore_dots(tasks_processed);

    let task1 = GeometryTaskAnalyser::get_task_data(tasks_with_dots);
    let res = Solver::solve_geometry_task(&task1.0, &task1.1);

    let project_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable not set");
    let project_path = PathBuf::from(project_dir).join("src");

    match Solver::save_steps_to_json(res, project_path.join("rectangles.json")) {
        Ok(_) => {
            let output = Command::new("python")
                .arg(project_path.join("plot_rectangles.py"))
                .arg(project_path.join("rectangles.json"))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .expect("Failed to execute Python script");

            if !output.status.success() {
                eprintln!("Python script error: {}",
                          String::from_utf8_lossy(&output.stderr));
            } else {
                println!("Python script output: {}",
                         String::from_utf8_lossy(&output.stdout));
            }
        }
        Err(e) => println!("Error saving JSON: {:?}", e),
    }
}