use std::fs::{File, canonicalize};
use std::env::home_dir;
use std::io::Read;
use reqwest;

"about" => {
    println!("u is a clone of U that provides random tools");
}

"get a wallpaper" => |resolution, category| {
    let wall_dir = home_dir().unwrap().join("Pictures/Wallpapers");
    let wall_path = canonicalize(wall_dir).expect("Wallpaper directory does not exist");
    println!("getting a wallpaper to put in {}!", wall_path.display());

    let url = format!(
        "https://alpha.wallhaven.cc/search?q={}&categories=100&purity=100&resolutions={}&sorting=random&order=desc",
        category,
        resolution,
    );
    let mut content = String::new();
    let mut resp = reqwest::get(&url).unwrap();
    assert!(resp.status().is_success());
    resp.read_to_string(&mut content);

    let id_loc = content.find("data-wallpaper-id=").unwrap() + 19;
    let id_end = content[id_loc..].find('"').unwrap() + id_loc;
    let id = &content[id_loc..id_end];

    let mut output = File::create(wall_path.join("output.jpg")).unwrap();
    let url = format!("https://wallpapers.wallhaven.cc/wallpapers/full/wallhaven-{}.jpg", id);
    let mut resp = reqwest::get(&url).unwrap();
    assert!(resp.status().is_success());

    resp.copy_to(&mut output).unwrap();

    println!("resolution: {}, category: {}", resolution, category);
    println!("id: {}", id);
}
