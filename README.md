# bodyproblem

A simple n-body problem simulator.

Uses RK4 with an extra step to account for conservation of energy.

Accepts the argument `-d` to set "density" (higher values make bodies smaller), and `-G` to set the universal gravitational constant.

Bodies are passed in through the format `x y vx vy m`, where x and y are the coordinates, vx and vy are the velocity, and m is the mass.

Selected cool inputs:
- Symmetric dancers `-d 15 -G 1.4   1 0 0 1.5 1 0   1 -0.5 0 1 -1   0 0 -1.5 1 0   -1 0.5 0 1`
- Asymmetric dancers `-d 15 -G 1.4   1 0 -0.1 1.5 1   0 1 -0.5 0 1   -1 0 0 -1.5 1   0 -1 0.6 0 1`

Note that in spite of the effort I make to conserve energy, it is not always conserved.
In particular, what I call an "orbital bomb" is a configuration where two bodies come into a very close binary orbit.
In this configuration, often energy just appears out of nowhere, leading to an "explosion" where the two bodies launch away from each other.
Both of the "dancer" inputs above have orbital bombs.

### Keybinds
`-` zooms out, `+` and `=` zoom in, spacebar pauses and resumes, and the enter key prints out a listing of quantities that should be conserved-ish.
Those quantities are: total energy, center of mass (should be conserved if the total initial momentum is 0), total momentum, and a few angular momenta.
