# Purpose
Schematic2BLS is an in-development tool used to convert a Minecraft schematic file (.schematic) into a Blockland Save File (.bls).

This tool was created to do a last attempt at finishing my [Mario 64 Project](https://www.youtube.com/watch?v=quJs0Zgsrjw) on Blockland by myself, with the goals of saving time, and applying what I have learned from my Computer Science major in university from undergrad to graduate school.

# Caveats
As of the time of this writing, Schematic2BLS will only recognize Stone blocks in a .schematic file when reconstructing a model in Blockland.

In addition, to render the result on Blockland, you will need to install two Add-Ons that add 2x and 1x Cubes respectively, which can be found here:
- [2x Cube](https://rtb.daprogs.com/forum.returntoblockland.com/dlm/viewFile7a46.html?id=2476)
- [1x Cube Top and Bottom](https://blocklandglass.com/addons/addon/1457)

# Usage
Schematic2BLS can be used during development as a CLI application. To use it, you will have to download the source code, compile the code, and then run the executable.

## Steps
1. Clone this repository via git, or download the ZIP file via clicking the Code button on GitHub.
2. Download (Rust)[] if you have not already.
3. Navigate to where you downloaded/unzipped the source code, and run `cargo build --release`.
4. Navigate to the newly built executable under the directory `target/release`.
5. Run the command `./schematic2bls <path_to_schematic> [scaling_factor]`, where
    - `<path_to_schematic>` is the location of your schematic file to be translated.
    - `[scaling_factor]` (Optional) is an integer on how much you would like to scale up the model.
6. Observe a newly created file called `output.bls` in your current directory.

