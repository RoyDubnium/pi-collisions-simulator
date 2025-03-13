# pi-collisions-simulator
Simulates the phenomenon where two objects colliding on a plane generate the digits of pi.
More information on this can be found [here](https://www.youtube.com/watch?v=6dTyOl1fmDo)
## How it works
To prevent imprecision from using exact time steps, this program computes the time until the next collision and then steps forward by exactly that much time. For animating, the program keeps track of the positions and velocities between each collision, and use the velocities to interpolate positions. For storing these in a manner such that it is easy to find the last collision, I use the [aatree](https://docs.rs/aatree/latest/aatree/) crate.
As this requires exponentially more computation for each extra digit, don't try calculating more than nine. You can try, but it may take a while.
### Minor Disclaimer
While the simulation code is entirely my own work, the animation code is written with AI assistance. 