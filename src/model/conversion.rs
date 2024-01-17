use crate::largest_cube::mapping::{idx_3d_from, GridSizes};
use nbt::CompoundTag;

pub fn schematic_to_3dgrid(schematic_root: CompoundTag) -> Vec<Vec<Vec<bool>>> {
    let length = schematic_root
        .get_i16("Length")
        .expect("Could not find Length field.") as usize;
    let width = schematic_root
        .get_i16("Width")
        .expect("Could not find Width field.") as usize;
    let height = schematic_root
        .get_i16("Height")
        .expect("Could not get Height field") as usize;

    let mut grid = vec![vec![vec![false; width]; height]; length];

    let blocks = schematic_root
        .get_i8_vec("Blocks")
        .expect("Could not get Blocks field in schematic.");
    let grid_size = GridSizes {
        x_len: length,
        y_len: width,
        z_len: height,
    };

    for (blocks_idx_1d, block_entry) in blocks.iter().enumerate() {
        let (i, j, k) = idx_3d_from(blocks_idx_1d, &grid_size);
        grid[j][k][i] = *block_entry == 1;
    }

    grid
}

#[cfg(test)]
mod tests {
    use nbt::decode::read_gzip_compound_tag;

    use super::*;
    use std::io::Cursor;

    fn load_test_schematic() -> CompoundTag {
        let mut file_cursor =
            Cursor::new(include_bytes!("../../assets/peachs_castle_8.schematic").to_vec());
        read_gzip_compound_tag(&mut file_cursor).expect("Could not read given schematic file.")
    }

    fn get_num_blocks_filled(grid: &Vec<Vec<Vec<bool>>>) -> usize {
        let mut num_blocks = 0;

        for length_entry in grid {
            for width_entry in length_entry {
                for entry in width_entry {
                    if *entry {
                        num_blocks += 1;
                    }
                }
            }
        }

        num_blocks
    }

    #[test]
    fn correct_num_blocks_loaded() {
        let schematic = load_test_schematic();
        let grid = schematic_to_3dgrid(schematic);

        let expected = 182446;
        let actual = get_num_blocks_filled(&grid);

        assert_eq!(expected, actual);
    }
}
