#!/usr/bin/env python3

import os
import sys
import traceback

import subprocess
import shutil
from pathlib import Path

import pyautogui

# NOTE: This is the variable to change depending on
# - Your Operating System
# - Where Steam is installed.
STEAM_PATH = Path('C:\Program Files (x86)\Steam\steamapps')
STEAM_APPS_PATH = STEAM_PATH.joinpath('common')
STEAM_COMPATDATA_PATH = STEAM_PATH.joinpath('compatdata')

PROTON_EXECUTABLE_PATH = STEAM_APPS_PATH.joinpath('Proton - Experimental/proton')

BLOCKLAND_PATH = STEAM_APPS_PATH.joinpath('Blockland')
BLOCKLAND_CONSOLE_PATH = BLOCKLAND_PATH.joinpath("console.log")
BLOCKLAND_SAVE_PATH = BLOCKLAND_PATH.joinpath("saves")
BLOCKLAND_EXECUTABLE_PATH = BLOCKLAND_PATH.joinpath("Blockland.exe")

DELTA_DEBUGGING_ASSETS_PATH = Path('../assets/delta_debugging')

# This was found by opening a bls file and
# identifying the line number of the "Linecount x"
# string.
LINECOUNT_LINE_NUMBER = 68 - 1

def getRunCommandForBlockland():
    """Returns a command with arguments needed to run
    Blockland represented as an array.
    """
    # A dedicated server is preferred so that we can just type
    # in commands, versus having to rely off of UI elements.
    # 
    # Reference: https://blockland.fandom.com/wiki/How_to_host_a_dedicated_server_in_Blockland.
    blockland_executable = [BLOCKLAND_EXECUTABLE_PATH, "ptlaaxobimwroe", "-dedicated"]
    if os.name == 'nt':
        return blockland_executable

    # Windows doesn't need the Proton compatibility layer, since
    # we're running native Windows code at this rate.
    os.environ["STEAM_COMPAT_DATA_PATH"] = str(STEAM_COMPATDATA_PATH)
    os.environ["STEAM_COMPAT_CLIENT_INSTALL_PATH"] = str(STEAM_COMPATDATA_PATH)

    proton_command = [str(PROTON_EXECUTABLE_PATH), "run"]

    return proton_command + blockland_executable

def runBinvox(objFile: Path) -> Path:
    """Returns the Path to a generated Minecraft Schematic File
    after running the binvox executable.

    Keyword arguments:
    objFile -- Path to the 3D Model File (.obj) used for conversion.
    """
    command = [".\\binvox.exe"]
    arguments = ["-e", "-rotz", "-t", "schematic", str(objFile)]

    subprocess.run(command + arguments, check=True)

    return Path(DELTA_DEBUGGING_ASSETS_PATH.joinpath(objFile.stem + ".schematic"))

def runSchematic2BLS(schematicFile: Path, scale: int) -> Path:
    """Returns the path of the generated Blockland Save file after running
    schematic2BLS.

    Keyword arguments:
    schematicFile -- Path to Minecraft Schematic File to be converted.
    scale -- The minimum size cube to use during conversion.
    """
    command = ["cargo", "run", "--release"]
    arguments = ["--", str(schematicFile), str(scale)]

    subprocess.run(command + arguments, check=True)

    return Path('output.bls').absolute()

def moveToSaves(outputPath: Path):
    """Copies the Blockland Save File located at the outputPath to
    Blockland's Save directory.

    Keyword arguments:
    outputPath -- Path to Blockland Save File.
    """
    shutil.copy(outputPath, BLOCKLAND_SAVE_PATH)

def listenUntil(process: subprocess.Popen, message: str):
    """Halts the program until the given process
    yields a certain message.

    Keyword arguments:
    process -- A subprocess instance that's currently running.
    message -- The string used to check whether it's okay to proceed.
    """
    while True:
        line = process.stdout.readline().rstrip()
        if message in line:
            break

def loadSave(saveFileName: str):
    """Returns the process referencing Blockland after Loading the Save File.

    Keyword arguments:
    saveFileName -- The name of the Save file to load.
    """
    command = getRunCommandForBlockland() 

    # NOTE: It is assumed that the game is not running by default,
    # so we start the application and wait until it is fully loaded.
    blocklandProcess = subprocess.Popen(command, encoding='ascii', stdout=subprocess.PIPE, text=True)

    # This is the last line that shows up in the
    # Blockland console when the game is loaded
    listenUntil(blocklandProcess, "Dedicated server is now running.")

    # Loading Saves from the server is possible due to the following Blockland Forum post:
    # https://forum.blockland.us/index.php?topic=294753.0
    pyautogui.typewrite('serverDirectSaveFileLoad("saves/{}", 3, "", 1);'.format(saveFileName))
    pyautogui.press('enter')

    # The server has no way of indicating when we're done,
    # so I had to time it manually for my specific schematic
    # file to approximate when it _should_ be done.
    listenUntil(blocklandProcess, "LOADING BRICKS")
    pyautogui.sleep(10.0)

    return blocklandProcess

def readBrickTotalFrom(outputPath: Path) -> int:
    """Returns the Total Brick Count located in a Blockland Save File.

    Keyword arguments:
    outputPath -- Path to Blockland Save File (.bls)
    """
    with open(outputPath) as saveFile:
        saveContent = saveFile.read().splitlines()

    linecountLine = saveContent[LINECOUNT_LINE_NUMBER]
    brickTotal = int(linecountLine.split()[1])
    return brickTotal

def captureBrickCountFromBlockland(blocklandProcess: subprocess.Popen[bytes]) -> int:
    """Returns the Brick Count read from a running instance of
    Blockland.

    Keyword arguments:
    blocklandProcess -- Reference to the subprocess running Blockland.
    """
    pyautogui.typewrite('echo(getBrickCount());')
    pyautogui.press('enter')

    brickCount = 0
    while True:
        line = blocklandProcess.stdout.readline().rstrip()
        if line.isdigit():
            brickCount = int(line)
            break

    return brickCount

def getBrickCounts(blocklandProcess: subprocess.Popen[bytes], outputPath: Path) -> tuple[int, int]:
    """Returns the Brick Count and Brick Total from a running instance of Blockland.

    Keyword arguments:
    blocklandProcess -- Reference to the subprocess running Blockland.
    outputPath -- Path to the Blockland Save File (.bls)
    """
    brickCount = captureBrickCountFromBlockland(blocklandProcess)
    brickTotal = readBrickTotalFrom(outputPath)

    return (brickCount, brickTotal)

def checkForHoles(objFile: Path, scale: int) -> bool:
    """Returns whether a converted Minecraft Schematic to a Blockland Save
    contains holes when running schematic2BLS.

    Keyword arguments:
    objFile -- Path to the 3D Model File (.obj) to convert into a Blockland Save.
    scale -- The Brick Cube size to be used for each block from Minecraft.
    """
    schematicFile = runBinvox(objFile)
    outputPath = runSchematic2BLS(schematicFile, scale)
    moveToSaves(outputPath)
    
    saveFileName = outputPath.name
    blocklandProcess = loadSave(saveFileName)
    brickCount, brickTotal = getBrickCounts(blocklandProcess, outputPath)
    blocklandProcess.terminate()
    
    return brickCount < brickTotal

if __name__ == '__main__':
    numArguments = len(sys.argv)
    objFilePath = DELTA_DEBUGGING_ASSETS_PATH.joinpath("peachs_castle.obj").absolute().resolve()
    if numArguments == 2:
        print("{}: Setting schematic file input to {}".format(sys.argv[0], sys.argv[1]))
        objFilePath = Path(sys.argv[1]).absolute().resolve()

    try:
        hasHoles = checkForHoles(objFilePath, 4)
        if hasHoles:
            exit(0)
        else:
            exit(1)
    except Exception:
        traceback.print_exc()
        exit(1)
