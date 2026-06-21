use rikuuid::v4::uuid_v4;

fn main() {
    let uuid = uuid_v4().unwrap();
    println!("{}", uuid);
}
