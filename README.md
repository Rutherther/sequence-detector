# Sequence detector
A simple cli tool that detects a predefined sequence of commands and performs operations (executing a command)
after a sequence is matched. There will be multiple commands associated with the sequence and only some of them will be
so called "debounce" commands that will start the sequence. Others will perform associated operation with just one key and exit.
Debounce keys will wait a few seconds before quitting, either another command will be pressed.

The way to detect a debounce key, is to look into the predefined sequences and look whether more than one sequence is matched.
If more than one sequence is matched, the current key is a debounce key and the program will run for debounce time after processing.
If no other key was pressed, then just execute the command of the current matched sequence (if there is any), else just quit
and let the next levels handle the command.

The program will use a temporary file for storing the current sequence, along with time of last command.

## Example application
I am going to use this tool for playing and pausing specific players on my computer.
I have the possibility to send some keys from my headphones (XF86AudioPlay, XF86AudioPause, XF86AudioNext, XF86AudioPrev, XF86AudioNext, XF86AudioLowerVolume, XF86AudioRaiseVolume).
I want to be able to tell my computer only from these keys, to start playing or stop playing either firefox or spotify. This is not possible without a sequence detector, as only
one of these could be paused or played at a time (could be hardcoded or the last playing etc.).

### Actual sequence explanation
1. standalone keys play, pause, next, prev etc. will just do what they are meant for.
2. keys next and prev will act as a debounce keys, they will start a new sequence.
3. volume up/down will finish the command. - next, up will toggle spotify, next, down will toggle firefox etc.

My config for this:
``` json
{
    "debounce_time": 2000,
    "groups": [
        {
            "group_id": "mpris",
            "sequences": [
                { "keys": ["next"], "action": "/home/ruther/Documents/my_projects/utils/mpris-ctl/cli/target/debug/mpris-ctl --player spotify next" },
                { "keys": ["prev"], "action": "/home/ruther/Documents/my_projects/utils/mpris-ctl/cli/target/debug/mpris-ctl --player spotify prev" },
                { "keys": ["play"], "action": "/home/ruther/Documents/my_projects/utils/mpris-ctl/cli/target/debug/mpris-ctl play" },
                { "keys": ["pause"], "action": "/home/ruther/Documents/my_projects/utils/mpris-ctl/cli/target/debug/mpris-ctl pause" },
                { "keys": ["volup"], "action": "amixer set Master 10%+ unmute" },
                { "keys": ["voldown"], "action": "amixer set Master 10%- unmute" },
                { "keys": ["next", "next"], "action": "/home/ruther/Documents/my_projects/utils/mpris-ctl/cli/target/debug/mpris-ctl --player firefox toggle" },
                { "keys": ["next", "prev"], "action": "/home/ruther/Documents/my_projects/utils/mpris-ctl/cli/target/debug/mpris-ctl --player spotify toggle" }
            ]
        }
    ]
}
```
