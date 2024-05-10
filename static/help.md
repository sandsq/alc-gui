# Advanced layout calculator
Please report issues or make suggestions at [https://github.com/sandsq/alc-gui](https://github.com/sandsq/alc-gui)!

## Overview
This app is designed to optimize a keyboard layout under various configuration options. 

Each position in the layout is given an "effort value", representing the relative effort required to reach that position. Each value is also paired with a hand+finger assignment, which is used to determine whether a given typing sequence results a favorable action such as hand alternation or finger rolling.

To find an optimized layout, the input is copied and randomized multiple times to form a population of layouts. Each layout in this population is then scored against various text datasets. The best layouts, i.e., those layouts that require the least amount of effort to type the text data, are further duplicated and modified to form a new generation of layouts. This process continues for a set number of generations and the final best 10 layouts are saved.

## The layout
The `Layout` tab lets the user customize the starting layout. This does not need to be filled out fully, as the optimizer will insert keycodes into blank slots. However, the user must take care to ensure that every layer is reachable by using the `LSX` keycodes. For example, `LS1` on layer 0 means pressing that key while layer 0 is active will switch to layer 1. Selecting an `LS` keycode will automatically place a corresponding keycode in the target layer as well. So continuing the previous example, `LS1` will automatically appear in layer 1 in the corresponding location. For more information regarding layers, check the [QMK documentation](https://docs.qmk.fm/#/feature_layers?id=layers). Layer switches in the optimizer are designed to act as one-shot switches or momentary switches, depending on configuration.

There are two additional flags per key. The lock can be toggle to ensure that the specified keycode will be locked to its specific position. This is useful if the user has certain muscle memory, keyboard shortcuts, etc. that they wish to preserve. The symmetry flag can be toggled to ensure that the two specified keys always remain symmetric to each other. The optimizer can still move their position, but the movement will not break symmetry. This is useful for modifiers like shift or for various brackets. One feature in the works is being able to constrain certain keys relative to each other. For example, the user may want to place corresponding brackets adjacent to each other, rather than symmetrically across the board.

Arbitrary layout sizes are not supported for now (because I decided to try out Rust's constant generics to specify array sizes, oops). Instead, choose the next largest layout size and block unused keys using the lock toggle. This approach can also be used to create irregular layout shapes, such as ergo ones. Non-grid layouts can still be optimized by mapping each key to a grid position -- there just currently is no visual indicator for non-grids.

## The effort layer
The `Effort layer` tab lets the user customize the effort layer. The values represent the relative effort required to reach a certain position -- higher value means more effort needed. As a rule of thumb, give the most accessible locations a value of 1 and scale the remaining positions accordingly. Some experimentation may be required to find numbers that suit the user.

## Hand assignments
The `Hand assignment` tab lets the user customize hand and finger assignments per key. These are use to determine whether sequences result in hand alternation or finger rolls, two favorable typing actions that can decrease the effort required to type said sequence. `Joint` refers to that part of the palm that meets the base of the pinkie, as some typists use that part of their hand to hit the keys in the bottom left / bottom right corners. Default hand assignments follow the homerow style of typing, but the user can change that to suit their needs.

## Config options
Hover over the `?` mark accompanying each option to read about it.

## Misc info
Using the [gruvbox color scheme](https://github.com/morhetz/gruvbox).