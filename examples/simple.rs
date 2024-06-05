use mini_uuid::MiniUuid;

fn main() {
  let mini = MiniUuid::new();

  println!("My uuid: {}", mini.to_string());
}
