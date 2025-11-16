# ori-archipelago-hint

This tool replicates the hint system from the standalone ori randomizers for archipelago: instead of telling you which location holds a specific item, it tells you in which area its located (or the game its in if this tool doesn't have the necessary informations).

![Screenshot of the app](https://i.imgur.com/3DUleJM.png)

## How to use

1. Unzip the file from the [release section](https://github.com/Foopyo/ori-archipelago-hint/releases)
2. Launch ori-archipelago-hint.exe
3. Click on Open File and select your spoiler log
4. Click on the hints button to reveal in which areas are the respective skills

If the app crashes upon loading your spoiler log, there's most likely an issue with the formating of locations.json (which should be placed next to ori-archipelago-hint.exe).

## Intended utilisation

Since this is supposed to replicate the hint system from the standalone Ori randomizers, its also supposed to replicate its rules (but feel free to ignore them)

1. Ori and the Blind Forest rules:
  - Click one time on Key Hints each time you collected 3 trees
  - Click on Forlorn hint once you completed the Forlorn Escape
2. Ori and the Will of the Wisps rules:
  - Click on Twillen hint once you can talk to them and have ~1000 Spirit Light total
  - Click on Opher hint once you can talk to them and have ~2000 Spirit Light total
  - Click on Lupo hint once you can talk to them in their house in Glades and have ~4000 Spirit Light total

## Supporting other games

While you can't add hints for items in other games without editing the source code, you can pretty easily add the area information from other games. This makes it possible to hint at the area within the game instead of hinting at the game a iven item is in.

To do so, edit locations.json to add any relevant informations. Format:
```
[
  {
  	"game_name": "Name of the game as it appears in the yaml file]",
  	"areas":[
      {
        "area_name": "Name of the area, this is the one getting displayed in the tool",
        "locations": [
          "name of pickup as it appears in the spoiler log",
          "second pickup",
          "third pickup"
        ]
      },
      {
      	"area_name": "second area",
      	"locations": [
          "forth pickup",
          "fifth pickup"
      	]
      }
  	]
  },
  {
  	"game_name": "game 2",
  	"areas":[
      {
        [...]
      }
  	]
  }
]
```
