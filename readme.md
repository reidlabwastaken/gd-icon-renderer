# gd-icon-renderer

rust geometryd ash icon redner!!

## usage

Provide your `GJ_GameSheet02-uhd`, `GJ_GameSheetGlow-uhd`, `Robot_AnimDesc2`, and `Spider_AnimDesc2` files along with their corresponding `*.plist` files.

## todo

- maybe use custom plist parser
- spider + robot support
- make `get_sprite_from_loaded` and `get_sprite` merged into `get_sprite`. i think this needs traits or something
- trim empty alpha space (robtop didnt make the bounds correctly :sob:)