

This program allows you to use a playstation5 dualsense controller to move your mouse and interact with your pc.


### compatibility:
- It work both on bluetooth and usb mode.

### keybindings:
## ps5 controller to pc-controls mappings
| action | controller |
|:-------------------------|:-----------------------|
LOCK-TOGGLE [hold-down 10s] | controller.button.Share / Create
mouse.xy (var speed) | controller.joystick.L
arrow-keys (var speed) | controller.joystick.R
arrow-keys (1x) | controller.DPad
mouse.L | controller.button.X
mouse.R | controller.button.Square
SHIFT | controller.button.O
CTRL | controller.button.Triangle
PageUp | controller.R2
PageDown | controller.L2
BrowserBack | controller.L1
BrowserForward | controller.R1

I used this as reference material for the byte-mappings of the controller input/output reports
> [github.com/nondebug/dualsense](https://github.com/nondebug/dualsense)
a copy of the html code is [here](./inspire/README.md) which was a huge help.\
specifically, because I was able to copy some important parts of the JS code and use it with only small modifications to the syntax.
