# gd-icon-renderer

A rust Geometry Dash icon renderer. Shhout out to [gd-icon-renderer](https://github.com/oatmealine/gd-icon-renderer), this project is just a rewrite but not in libvips and crystal

## usage

Provide your `GJ_GameSheet02-uhd`, `GJ_GameSheetGlow-uhd`, `Robot_AnimDesc2`, and `Spider_AnimDesc2` files along with their corresponding `*.plist` files.

## todo

- swap to a custom plist parser
- change zany anim to argument
- trim empty alpha space
- i think theres some weird shifting and offsets going on. investigate plz. really big on spider 16 for some reason???? related issue here: [issue #2, oatmealine/gd-icon-renderer](https://github.com/oatmealine/gd-icon-renderer/issues/2)