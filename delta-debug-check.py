#!/usr/bin/env python3

import sys
import traceback

import subprocess
import shutil
from pathlib import Path

import pyautogui

BLOCKLAND_PATH = Path('C:\Program Files (x86)\Steam\steamapps\common\Blockland')
BLOCKLAND_SAVE_PATH = BLOCKLAND_PATH.joinpath("saves")
BLOCKLAND_EXECUTABLE_PATH = BLOCKLAND_PATH.joinpath("Blockland.exe")

DELTA_DEBUGGING_ASSETS_PATH = Path('assets/delta_debugging')

# This was found by opening a bls file and
# identifying the line number of the "Linecount x"
# string.
LINECOUNT_LINE_NUMBER = 68 - 1

def runSchematic2BLS(schematicFile: str, scale: int) -> Path:
    """Returns the path of the generated Blockland Save file after running
    schematic2BLS.

    Keyword arguments:
    schematicFile -- Path to Minecraft Schematic File to be converted.
    scale -- The minimum size cube to use during conversion.
    """
    command = ["cargo", "run", "--release"]
    arguments = ["--", schematicFile, str(scale)]

    subprocess.run(command + arguments, check=True)

    return Path('output.bls').absolute()

def moveToSaves(outputPath: Path):
    """Copies the Blockland Save File located at the outputPath to
    Blockland's Save directory.

    Keyword arguments:
    outputPath -- Path to Blockland Save File.
    """
    shutil.copy(outputPath, BLOCKLAND_SAVE_PATH)

def loadSave():
    """Returns the process referencing Blockland after Loading a Save File.
    """
    command = [str(BLOCKLAND_EXECUTABLE_PATH)]

    # NOTE: It is assumed that the game is not running by default,
    # so we start the application and wait until it is fully loaded.
    blocklandProcess = subprocess.Popen(command, encoding='ascii', stdout=subprocess.PIPE, text=True)
    while True:
        line = blocklandProcess.stdout.readline().rstrip()
        # This is the last line that shows up in the
        # Blockland console when the game is loaded
        # and waiting on the Main Menu.
        if 'Authentication SUCCESS' in line:
            break

    # Blockland cannot handle the raw speed of pyautogui, so
    # there has to be a delay.
    mouseClickWaitSecs = 1

    # All of this gets us into a Single Player instance
    # of Blockland ready to go.
    startButtonX, startButtonY = (697, 502)#pyautogui.locateCenterOnScreen(str(DELTA_DEBUGGING_ASSETS_PATH.joinpath('startgamebutton.png')))
    pyautogui.click(x=startButtonX, y=startButtonY, duration=mouseClickWaitSecs)

    selectButtonX, selectButtonY = (1996, 1207)#pyautogui.locateCenterOnScreen(str(DELTA_DEBUGGING_ASSETS_PATH.joinpath('selectbutton.png')))
    pyautogui.click(x=selectButtonX, y=selectButtonY, duration=mouseClickWaitSecs)
    # The Launch Game button happens to be in the same
    # location, so we just click where we are again.
    pyautogui.click()

    while True:
        line = blocklandProcess.stdout.readline().rstrip()
        # This is the last line that shows up in the
        # Blockland console when loaded into a Single Player
        # instance.
        if "Linking GLSL program" in line:
            break
    
    # And the finale, loading the Bricks. This loads the most recent save
    # from the game, which is assumed to be at the top and automatically
    # selected.
        
    # Testing locally, once again we found a case where we outspeed
    # Blockland, so we have to wait for it to catch up again.
    pyautogui.typewrite(['esc'])
    loadButtonX, loadButtonY = (1220, 785)#pyautogui.locateCenterOnScreen(str(DELTA_DEBUGGING_ASSETS_PATH.joinpath('loadbutton.png')))
    pyautogui.click(x=loadButtonX, y=loadButtonY, duration=mouseClickWaitSecs)

    loadSaveButtonX, loadSaveButtonY = (1544, 922)#pyautogui.locateCenterOnScreen(str(DELTA_DEBUGGING_ASSETS_PATH.joinpath('loadsavebutton.png')))
    pyautogui.click(x=loadSaveButtonX, y=loadSaveButtonY, duration=mouseClickWaitSecs)

    while True:
        line = blocklandProcess.stdout.readline().rstrip()
        if "LOADING BRICKS" in line:
            pyautogui.sleep(10.0)
            break

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
    pyautogui.shortcut('alt', 'tab')
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

def checkForHoles(schematicFile: str, scale: int) -> bool:
    """Returns whether a converted Minecraft Schematic to a Blockland Save
    contains holes when running schematic2BLS.

    Keyword arguments:
    schematicFile -- Path to the Minecraft Schematic to convert into a Blockland Save.
    scale -- The Brick Cube size to be used for each block from Minecraft.
    """
    outputPath = runSchematic2BLS(schematicFile, scale)
    moveToSaves(outputPath)
    
    blocklandProcess = loadSave()
    brickCount, brickTotal = getBrickCounts(blocklandProcess, outputPath)
    blocklandProcess.terminate()
    
    return brickCount < brickTotal

if __name__ == '__main__':
    numArguments = len(sys.argv)
    schematicFilePath = "assets/peachs_castle_4.schematic"
    if numArguments == 2:
        print("{}: Setting schematic file input to {}".format(sys.argv[0], sys.argv[1]))
        schematicFilePath = sys.argv[1]

    try:
        hasHoles = checkForHoles(schematicFilePath, 4)
        if hasHoles:
            exit(0)
        else:
            exit(1)
    except Exception:
        traceback.print_exc()
        exit(1)