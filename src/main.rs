mod game;

#[tokio::main]
async fn main() {
    let u = uuid::Uuid::new_v4();
    let u2 = uuid::Uuid::new_v4();
    let u3 = uuid::Uuid::new_v4();
    let p = game::PlayerId(u);
    let p2 = game::PlayerId(u2);
    let p3 = game::PlayerId(u3);

    let mut s = game::create_game(p);
    s.join(p2);
    s.join(p3);
    println!("{:?}", s);
}
