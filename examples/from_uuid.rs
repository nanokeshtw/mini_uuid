use mini_uuid::MiniUuid;
use uuid::Uuid;

fn main() {
  let uuid = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
  let mini_uuid = MiniUuid::from_uuid(uuid);

  println!("{}", uuid.to_string());
  println!("{}", mini_uuid.to_string());
}
