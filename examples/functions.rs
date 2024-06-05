use mini_uuid::MiniUuid;

fn main() {
  let mini = MiniUuid::new();
  println!("New mini {}", mini.to_string());
  let mini = MiniUuid::from_uuid_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
  println!("From Uuid str {}", mini.to_string());
  let mini = MiniUuid::from_string("Z-VQRBCxQm-SR7toDl_gyA==").unwrap();
  println!("From mini str {}", mini.to_string());
  let mini = MiniUuid::from_url_base64("Z-VQRBCxQm-SR7toDl_gyA==").unwrap();
  println!("From mini url str {}", mini.to_string());
  let mini = MiniUuid::from_url_base64_no_pad("Z-VQRBCxQm-SR7toDl_gyA").unwrap();
  println!("From mini url no pad str {}", mini.to_string());
  let mini = MiniUuid::from_base64("Z+VQRBCxQm+SR7toDl/gyA==").unwrap();
  println!("From mini str {}", mini.to_string());
  let mini = MiniUuid::from_base64_no_pad("Z+VQRBCxQm+SR7toDl/gyA").unwrap();
  println!("From mini no pad str {}", mini.to_string());

  let uuid = mini.to_uuid();
  println!("To Uuid {}", uuid.to_string());
  let uuid = mini.to_string();
  println!("To mini str {}", uuid);
  let uuid = mini.to_url_base64();
  println!("To mini url str {}", uuid);
  let uuid = mini.to_url_base64_no_pad();
  println!("To mini url no pad str {}", uuid);
  let uuid = mini.to_base64();
  println!("To mini str {}", uuid);
  let uuid = mini.to_base64_no_pad();
  println!("To mini no pad str {}", uuid);
}
