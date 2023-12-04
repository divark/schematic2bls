use super::Brick;

pub fn save_header() -> String {
    let warning_text = "This is a Blockland save file.  You probably shouldn't modify it cause you'll screw it up.";
    let mut header_body = "1\r\n\r\n".to_string();

    let data_lines = vec![
        "0.898039 0.000000 0.000000 1.000000",
        "0.898039 0.898039 0.000000 1.000000",
        "0.000000 0.498039 0.247059 1.000000",
        "0.200000 0.000000 0.800000 1.000000",
        "0.898039 0.898039 0.898039 1.000000",
        "0.749020 0.749020 0.749020 1.000000",
        "0.498039 0.498039 0.498039 1.000000",
        "0.200000 0.200000 0.200000 1.000000",
        "0.392157 0.192157 0.000000 1.000000",
        "0.901961 0.337255 0.078431 1.000000",
        "0.749020 0.176471 0.482353 1.000000",
        "0.384314 0.000000 0.113725 1.000000",
        "0.129412 0.266667 0.266667 1.000000",
        "0.000000 0.137255 0.329412 1.000000",
        "0.101961 0.458824 0.764706 1.000000",
        "1.000000 1.000000 1.000000 1.000000",
        "0.078431 0.078431 0.078431 1.000000",
        "1.000000 1.000000 1.000000 0.247059",
        "0.921569 0.513726 0.674510 1.000000",
        "1.000000 0.603922 0.419608 1.000000",
        "1.000000 0.874510 0.611765 1.000000",
        "0.956863 0.874510 0.784314 1.000000",
        "0.784314 0.921569 0.486275 1.000000",
        "0.537255 0.694118 0.549020 1.000000",
        "0.556863 0.929412 0.956863 1.000000",
        "0.694118 0.658824 0.901961 1.000000",
        "0.874510 0.556863 0.956863 1.000000",
        "0.666667 0.000000 0.000000 0.698039",
        "1.000000 0.498039 0.000000 0.698039",
        "0.988235 0.956863 0.000000 0.698039",
        "0.000000 0.470588 0.192157 0.698039",
        "0.000000 0.200000 0.639216 0.698039",
        "0.592157 0.156863 0.392157 0.694118",
        "0.549020 0.698039 1.000000 0.698039",
        "0.847059 0.847059 0.847059 0.698039",
        "0.098039 0.098039 0.098039 0.698039",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
    ];

    for data in data_lines {
        header_body.push_str(&format!("{}\r\n", data));
    }

    format!("{}\r\n{}", warning_text, header_body)
}

pub fn save_bricks(bricks: &Vec<Brick>) -> String {
    let linecount_line = format!("Linecount {}", bricks.len());

    let mut brick_contents = String::new();
    for brick in bricks {
        brick_contents.push_str(&format!("{}\r\n", brick));
        brick_contents.push_str("+-OWNER 999999\r\n");
    }

    format!("{}\r\n{}", linecount_line, brick_contents)
}

pub fn to_save_file_output(bricks: &Vec<Brick>) -> String {
    let save_header = save_header();
    let save_bricks = save_bricks(bricks);

    format!("{}{}", save_header, save_bricks)
}
