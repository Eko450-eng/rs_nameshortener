slint::include_modules!();

use std::fs;

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;
    let test = false;

    let _ = fs::create_dir_all("./in");
    let _ = fs::create_dir_all("./out");

    if test {
        for i in 0..10 {
            let name = format!("./in/newajhdlwahlkjdhakljwhdlkajhdklawjhdk{}", i);
            let _ = fs::write(name, "hey");
        }
    }

    let mut iteration = 0;

    ui.on_started({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let files = fs::read_dir("./in").unwrap();

            for file in files {
                let file_unwrapped = file.unwrap();
                let file_name = file_unwrapped.file_name();
                let file_name_string = file_name.to_string_lossy().to_string();

                let new_length_string = ui.get_cutoff_text().to_string();
                let mut new_length: usize = 50;

                match new_length_string.parse::<usize>() {
                    Ok(parsed_value) => {
                        new_length = parsed_value;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }

                let file_ending = ui.get_file_ending_selected();

                let new_name: String = file_name_string.chars().take(new_length - 3).collect();
                let new_path = format!("./out/{}_{}.{}", iteration, new_name, file_ending);

                let _ = fs::rename(file_unwrapped.path(), new_path);
                iteration += 1;
            }
        }
    });

    let _ = ui.run();
    Ok(())
}
