# gd-icon-renderer

rust geometryd ash icon redner!! shout out to [gd-icon-renderer](https://github.com/oatmealine/gd-icon-renderer) oat

## usage

Provide your `GJ_GameSheet02-uhd`, `GJ_GameSheetGlow-uhd`, `Robot_AnimDesc2`, and `Spider_AnimDesc2` files along with their corresponding `*.plist` files.

## todo

- use custom plist parser. current one takes like 5 seconds to parse an animation file
- change zany anim to argument
- trim empty alpha space (robtop didnt make the bounds correctly :sob:)
- i think theres some slight shifting in the transform to do with rotation. investigate plz. really big on spider 16 for some reason????