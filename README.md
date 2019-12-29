# Farm Together Calculator
A (currently super barebones) calculator to help you choose the crops with the best profit in a certain amount of time.

For example, if you loaded your farm and knew you were going to be playing in 8 hours time, the calculator will give you a list of the best crops to plant now, ready to be harvested in 8 hours.

# Usage
Right now everything is hardcoded. Add/remove crops to `crops.json`, change the `money` and `time` (minutes) values in `main.rs`, and `cargo run`.

# TODO
* Add parsing of time strings (e.g. 1h = 60, 2d = 2880)
* Add "watered" option (halving growing time)
* Add all crops
* Add option for only unlocked crops
* Add season support
* Add GUI