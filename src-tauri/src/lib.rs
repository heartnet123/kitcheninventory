use tauri_plugin_sql::{Migration, MigrationKind};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "Create tables for inventory app",
        sql: r#"
CREATE TABLE IF NOT EXISTS items (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name VARCHAR,
  category VARCHAR,
  quantity REAL,
  unit VARCHAR,
  cost REAL,
  cost_per_unit REAL,
  expiration_date DATE,
  location VARCHAR
);

CREATE TABLE IF NOT EXISTS inventory_transactions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  item_id INTEGER,
  transaction_type VARCHAR, -- "in" สำหรับรับเข้า, "out" สำหรับการใช้หรือขาย
  change_quantity REAL,
  transaction_date DATETIME,
  notes TEXT,
  FOREIGN KEY(item_id) REFERENCES items(id)
);

CREATE TABLE IF NOT EXISTS recipes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name VARCHAR,
  description TEXT,
  recipe_cost REAL,
  selling_price REAL,
  profit REAL,
  profit_margin REAL,
  image BLOB -- เพิ่มคอลัมน์สำหรับเก็บรูปภาพ
);

CREATE TABLE IF NOT EXISTS recipe_ingredients (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  recipe_id INTEGER,
  item_id INTEGER,
  quantity REAL,
  unit VARCHAR,
  FOREIGN KEY(recipe_id) REFERENCES recipes(id),
  FOREIGN KEY(item_id) REFERENCES items(id)
);

CREATE TABLE IF NOT EXISTS financial_records (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  record_type VARCHAR, -- "income" หรือ "expense"
  amount REAL,
  record_date DATETIME,
  description TEXT,
  recipe_id INTEGER, -- ระบุว่าสูตรไหนถูกใช้งานในรายการนี้
  quantity INTEGER,  -- จำนวนสินค้าที่ขาย (สำหรับสูตรที่บันทึกเป็น income)
  FOREIGN KEY(recipe_id) REFERENCES recipes(id)
);
"#,
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:inventory.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
