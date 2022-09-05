#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate csv;



#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_file_command(path: String) -> Result<Vec<i32>, ()>{
  let filepath = std::path::Path::new(&path);
  let content = match std::fs::read_to_string(filepath){
    Ok(content) => content,
    Err(e) => return Err(()),
  };

  let mut reader = csv::Reader::from_reader(content.as_bytes());
  let mut plotdata: Vec<i32> = vec![];
  for record in reader.records() {
      let record = record.unwrap();
      //println!("{}:::{}", &record[0], &record[1]);
      plotdata.push(record[0].trim().parse().unwrap());
      plotdata.push(record[1].trim().parse().unwrap());
  }

  println!("{:?}",plotdata);

  Ok(plotdata)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![read_file_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
