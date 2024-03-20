#!/usr/bin/env python3

import subprocess
import shutil
from pathlib import Path
import time

import pyautogui

BLOCKLAND_SAVE_PATH = Path('C:\Program Files (x86)\Steam\steamapps\common\Blockland\saves')
DELTA_DEBUGGING_ASSETS_PATH = Path('assets/delta_debugging')

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

def loadSave(outputName: str):
    """Returns the process referencing Blockland after Loading a Save File based on the outputName.

    Keyword arguments:
    -- outputName: Name of the Save to load in Blockland without the extension.
    """
    blocklandExecutablePath = Path('C:\Program Files (x86)\Steam\steamapps\common\Blockland\Blockland.exe')

    command = [str(blocklandExecutablePath)]

    # NOTE: It is assumed that the game is not running by default,
    # so we start the application and wait until it is fully loaded.
    blocklandProcess = subprocess.Popen(command, stdout=subprocess.PIPE)
    while True:
        line = blocklandProcess.stdout.readline().rstrip().decode("utf-8")
        if 'Authentication SUCCESS' in line:
            break

    # Blockland cannot handle the raw speed of pyautogui, so
    # there has to be a delay.
    mouseClickWaitSecs = 1

    # All of this gets us into a Single Player instance
    # of Blockland ready to go.
    startButtonX, startButtonY = pyautogui.locateCenterOnScreen(str(DELTA_DEBUGGING_ASSETS_PATH.joinpath('startgamebutton.png')))
    pyautogui.click(x=startButtonX, y=startButtonY, duration=mouseClickWaitSecs)

    selectButtonX, selectButtonY = pyautogui.locateCenterOnScreen(str(DELTA_DEBUGGING_ASSETS_PATH.joinpath('selectbutton.png')))
    pyautogui.click(x=selectButtonX, y=selectButtonY, duration=mouseClickWaitSecs)
    pyautogui.click()

    while True:
        line = blocklandProcess.stdout.readline().rstrip().decode("utf-8")
        if "Linking GLSL program" in line:
            break
    
    # And the finale, loading the Bricks. It's assumed that the top save
    # will be our output.
    time.sleep(2.0)
    pyautogui.typewrite(['esc'])
    loadButtonX, loadButtonY = pyautogui.locateCenterOnScreen(str(DELTA_DEBUGGING_ASSETS_PATH.joinpath('loadbutton.png')))
    pyautogui.click(x=loadButtonX, y=loadButtonY, duration=mouseClickWaitSecs)

    loadSaveButtonX, loadSaveButtonY = pyautogui.locateCenterOnScreen(str(DELTA_DEBUGGING_ASSETS_PATH.joinpath('loadsavebutton.png')))
    pyautogui.click(x=loadSaveButtonX, y=loadSaveButtonY, duration=mouseClickWaitSecs)

    return blocklandProcess

def checkForHoles(schematicFile: str, scale: int):
    outputPath = runSchematic2BLS(schematicFile, scale)
    moveToSaves(outputPath)
    
    outputName = outputPath.stem
    blocklandProcess = loadSave(outputName)

if __name__ == '__main__':
    try:
        checkForHoles("assets/peachs_castle_4.schematic", 4)
        exit(0)
    except Exception as e:
        print(e)
        exit(1)