use schematic2bls::*;

use std::env;
use std::path::Path;

fn main() {
    let execution_args: Vec<String> = env::args().collect();
    if execution_args.len() < 2 || execution_args.len() > 3 {
        eprintln!("Usage: schematic2bls <path_to_schematic> [scaling_factor]");
        return;
    }

    let model_arg = execution_args
        .get(1)
        .expect("schematic2bls: path_to_schematic not provided.");
    let model_path = Path::new(model_arg);
    let default_scaling_factor = 4;
    let scaling_factor = if let Some(scaling_arg) = execution_args.get(2) {
        scaling_arg.parse::<u8>().unwrap_or(default_scaling_factor)
    } else {
        default_scaling_factor
    };

    let model = load_schematic(model_path);
    let voxel_grid = parse_grid_from_model(model);
    let largest_cubes = extract_largest_cubes_from(voxel_grid, scaling_factor as u16);
    let bricks = extract_bricks_from(largest_cubes);

    let mut save_file_name = model_path
        .file_stem()
        .expect("schematic2bls: Could not get file stem from model path.")
        .to_os_string();
    save_file_name.push(".bls");

    let save_file_name = save_file_name.into_string().expect(
        "schematic2bls: Could not convert OS String into normal String for Save File Name.",
    );

    write_save_file(&bricks, save_file_name);
}
