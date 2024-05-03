# Purpose
Schematic2BLS is a tool used to convert a Minecraft schematic file (.schematic) into a Blockland Save File (.bls).

This tool was created to do a last attempt at finishing my [Mario 64 Project](https://www.youtube.com/watch?v=quJs0Zgsrjw) on Blockland by myself, with the goals of saving time, and applying what I have learned from my Computer Science major in university from undergrad to graduate school.

# Requirements
As of the time of this writing, Schematic2BLS will only recognize Stone blocks in a .schematic file when reconstructing a model in Blockland, since this was originally designed to be a program to run after making a schematic file using [binvox](https://www.patrickmin.com/binvox/).

In addition, this program supports scaling to smaller cubes that are not included in Blockland by default. Listed are some optional add-ons to include in your Blockland installation if scaling is below 4:
- [2x Cube](https://rtb.daprogs.com/forum.returntoblockland.com/dlm/viewFile7a46.html?id=2476)

# Usage
Schematic2BLS can be used as a CLI application. To use it, you will either have to download the [latest release executable](https://github.com/divark/schematic2bls/releases/latest) [RECOMMENDED], or download the source code, compile the code, and then run the executable.

Either way, it is recommended to run this program in a Terminal or Powershell environment, whichever is most appropriate for your Operating System. The way to run this program is as follows:

`./schematic2bls <path_to_schematic> [scaling_factor]`, where

- `<path_to_schematic>` is the location of your schematic file to be translated.
- `[scaling_factor]` (Optional) is the type of cube bricks to use by default for scaling purposes. Without specifying this, the default factor is 4.

Once executed, a newly created file called `<schematic_file_name>.bls` will be in the current directory.

## Compilation Steps 
1. Clone this repository via git, or download the ZIP file via clicking the Code button on GitHub.
2. Download [Rust](https://www.rust-lang.org/) if you have not already.
3. Navigate to where you downloaded/unzipped the source code, and run `cargo build --release`.
4. Navigate to the newly built executable under the directory `target/release`.

