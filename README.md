# Farm Together Calculator
A (currently super barebones) calculator to help you choose the crops with the best profit in a certain amount of time.

For example, if you loaded your farm and knew you were going to be playing in 8 hours time, the calculator will give you a list of the best crops to plant now, ready to be harvested in 8 hours.

# Usage
You may need to adjust `crops.json` (and make sure it exists). It's set to my currently unlocked crops.

Run the command with `money` and `time` arguments. If you have 100,000 money and want to harvest in 45 minutes:
`ft_calc 100000 45`.

# TODO
TODO, roughly in priority order.
* Make Crop struct reusable (e.g. for animals, flowers)
* Add a proper config
* Add "watered" option (halving growing time)
 * This should only apply to Crops
* Add option for only unlocked crops
* Add all crops
* Add GUI support
* Add season support
 * Useful if it's Winter and you want to finish playing before the end of the season

# Future
These could potentially be done in the future:
* Add parsing of time strings (e.g. 1h = 60, 2d = 2880) via command line
* Add optimisation calculations for XP / Gems
* Add fish, animals, and flowers
* Add chores