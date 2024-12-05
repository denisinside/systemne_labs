use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;
use lab2::parse;

fn main() -> Result<(), Box<dyn Error>> {
    let source =
        "позначити прямокутник ABCD;
         ЗДВИНУТИ прямокутник НА (-5, -3);
         визначити площу;
         змінити розмір у 2 рази;
         визначити периметр;
         Побудувати ДіАгональ прямокутника;
         позначити прямокутник ABGD з координатами (5,5), (5,10), (15,10), (15,5);
         перемістити прямокутник у (20, 20);
         позначити прямокутник SEKY;
         здвинути прямокутник на (7, -5);
         позначити перетин ABCD;
         здвинути прямокутник  на (17, -15);
         позначити прямокутник SL0N1K розміром 100 мм x 10 у точці (5, -20);
         побудувати M0H1LA на основі відношення 4:5 з діагоналлю 200 мм у точці (-10, 20);
         //переіменувати точку ABCD C на L;";

    let project_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable not set");

    let project_path = PathBuf::from(project_dir).join("src");
    println!("Project directory: {:?}", project_path);

    match parse(source, &project_path) {
        Ok(_) => {
            Command::new("python")
                .arg(project_path.join("plot_rectangles.py"))
                .arg(project_path.join("rectangles.json"))
                .spawn()?
                .wait()?;
        },
        Err(e) => println!("Error occurred: {:?}", e),
    }
    Ok(())
}
