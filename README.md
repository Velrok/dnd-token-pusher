# dnd-token-pusher

A simple D&D map engine with command support.

## TODO

* Manage two windows: DM view / Player view ✅
* Render background image ✅
* Render grid / Cell coordinates
* Token Part 1 (image, position, visible, size): Rendering, Commands
* Fog Of War (Rendering, Command Inputs: Reveal/Shadow)
* Command Input: Battlemap
* Load initial commands from file (`./dnd-token-pusher dungeon1_save20201229.txt`)

* Save state (full event log including initial commands from input file)
* Render dead tokens differently
* Initiative Order: (Rendering, Token attribute, Command)
* Command Input: tokens part 2 (health, max-health, damage)
* Command Input: dice rolls

## Concepts

* Battlemap (Image, Grid, Offset)
* Token (Sizes)
* Fog of War
* Initiate Order
* DM View / Player View
* Events (e.g. new tokens, define battlemap)
* Sync state via filesystem (via text file, see below)
* Battlemap use chess-style coordinates: `A1`, `C5` etc.

Maybe Later:

* Undo command
* Command history (arrow up to recall previous commands and allow edits)
* Distance & Area stencils (cone, qube, radius, ...)
* Objects (items on the floor, Doors, secret walls ...)
* Conditions (Concentration, Dead, Prone, ...)
* Token library (Predefined set of monster tokens with images & hp)? !! Licence issues?
* Animated tokens (gifs?)
* Spellslot Tracking
* Legendary Actions / Reactions / Resistances
* Mouse support (construct commands from mouse clicks example: clicking a token than a cell would build a move command)
* Soundscape (a list of audio files, to play as background music)?
* Dynamic visibility (calculate player visibility and FoW dynamically; would require info about solid objects)
* Networking (allow players to install a client and move their characters or send pointers)

## FileFormat Bainstorming

* text
* line by line
* optimized for human input
* easy / quick to type
* Lines should be easily copy & pastable => Each line is a complete command / event

## Example: 1 player, 1 monster

```bash
battlemap --url=https://i.redd.it/q2uayh37ndb41.png --width=12 --height=12
token goblinking --image=globin.png --name=Goblin --size=small --max-health=5 --pos=A1 --initiative=11
token barb --image=barbarian.png --name=Kuglor --size=medium --health=15 --max-health=22 --pos=D3 --visible
token goblinking --image=dead.png --health=0
token barb --pos=F2
token goblinwarrior --url=globin.png --name=Goblin --size=small --max-health=5 --pos=A5
```

Commands the DM can use directly during a session:

```bash
h barb 5 # heal (by amount)
m goblinking C6 # move (absolut)


reveal A2 F19 # top-left bottom-right

show goblinking # alias for 'token goblinking --visible'

# fight starts (goblinking already has an initiative in the file above)
i barb 15       # alias for 'token barb --initiative=15'
# also: show --all / hide --all

d barb 5 # damage (by amount)
d goblinking 10+5
hd barb 11 # (11 / 2 floored) => 5 damage => half damage / resistant
dd barb 20 # 40 damage => double damage / ciritical

# roll some die
2d8+5
```
