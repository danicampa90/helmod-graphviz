# Helmod to Graphviz dot
This tool allows to generate "nice" graphs of your production blocks and production chains that you built using [Helmod](https://mods.factorio.com/mod/helmod) in Factorio.

It uses the production chain you built in factorio and the recipe data from the game to built an accurate graph, that works with *your* currently installed mods, and not with only the most common mods.

## Requirements:
### Factorio:
- [Helmod](https://mods.factorio.com/mod/helmod) for planning your factory.
- [Data Exporter to JSON](https://mods.factorio.com/mod/recipelister) mod, for extracting recipes.json.

### For running the software:
- A recent installation of rust (that supports 2018 edition).
- graphviz
- (optional) gnu "make".

## Instructions for usage:
- Start Factorio, install the "data exported to JSON" mod.
- Start a *new* game. The data exporter only exports when starting a new game in factorio, not when loading an existing one. 
- find recipe.json in the script-output folder (look up where this folder is depending on if you are using windows or linux). copy that file in the root of the project.
- load up your savegame. open helmod end use the "export" button in there to copy the data about your production chain.
  - for now this requires some postprocessing: in a bash terminal execute "cat | base64 -d | gzip -d > test.txt", paste the exported text then press ENTER and CTRL+D
  - open test.txt and remove everything before the first "{" and everything after the last "}". Save the file
- run "cargo run" to generate the the dot files.
- run "make" to build PNG pictures of them.
