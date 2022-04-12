use mpris::PlayerFinder;

fn main() {
    println!("Hello, world!");
    let player = PlayerFinder::new()
        .expect("Couldnt connect to D-Bus")
        .find_active()
        .expect("Couldnt find player");

    let metadata = player.get_metadata().expect("COuld not get metadata");

    dbg!(metadata.title());

    let mut tracker = player.track_progress(1000).unwrap();

    loop {
        let (progress, was_changed) = tracker.tick();

        if was_changed {
            println!("title: {}", progress.metadata().title().unwrap());
            dbg!(progress);
        }
    }
}
