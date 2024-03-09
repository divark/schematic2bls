use schematic2bls::*;
mod blockland;

use std::env;

fn main() {
    let execution_args: Vec<String> = env::args().collect();
    if execution_args.len() < 2 || execution_args.len() > 3 {
        eprintln!("Usage: schematic2bls <path_to_schematic> [scaling_factor]");
        return;
    }

    let model_arg = execution_args
        .get(1)
        .expect("schematic2bls: path_to_schematic not provided.");
    let default_scaling_factor = 4;
    let scaling_factor = if let Some(scaling_arg) = execution_args.get(2) {
        scaling_arg.parse::<u8>().unwrap_or(default_scaling_factor)
    } else {
        default_scaling_factor
    };

    let model = load_schematic(&model_arg);
    let voxel_grid = parse_grid_from_model(model);
    let largest_cubes = extract_largest_cubes_from(voxel_grid, scaling_factor as u16);
    let bricks = extract_bricks_from(largest_cubes);

    write_save_file(&bricks);
}
