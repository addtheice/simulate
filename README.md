# Simulate

[![Join the chat at https://gitter.im/addtheice/simulate](https://badges.gitter.im/addtheice/simulate.svg)](https://gitter.im/addtheice/simulate?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![Build Status](https://travis-ci.org/addtheice/simulate.svg?branch=master)](https://travis-ci.org/addtheice/simulate) [![Clippy Linting Result](http://clippy.bashy.io/github/addtheice/simulate/master/badge.svg)](http://clippy.bashy.io/github/addtheice/simulate/master/log)
 [![Join the chat at https://gitter.im/addtheice/simulate](https://badges.gitter.im/addtheice/simulate.svg)](https://gitter.im/addtheice/simulate?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
 
Basic rust based simulation library.

Currently supporting:
  * 1 Dimensional electromagnetic simulation using FDTD supporting dielectric
  materials and free space.

I'm still working on use cases and how I want this API to work.

I've got two major use cases outlined so far:

Setup an environment with sources, dimensions, materials, and sampling moments.

This should allow for the common use case of setting up a simulation and then
running it for a specific period of time  while also being able to do things
like 'save this region of the field over the entire run' or
'check for these outlier conditions'.

The trick will be in creating a useful enough set of conditionals to make this
work which doesn't require extreme run time overhead, or severe complexity for
the user.

The most common example of this use would be testing waveguide designs for
example. Something that we need to run a full simulation over the full time
series and later analysis / visualization will be done on either all the data
or a subset of the data. The main point here is that the entire simulation
will be run no matter what happens during the simulation.

The second main use case is for 'real time' simulations where a simulation will
feed back during the run into a new selection. An example would be evolutionary
algorithms for antenna designs. The simulation should be sampled after every
time step and potentially stopped when a design can be shown to be incorrect. In
addition it will be useful for dynamic environments where the structure of the
environment may not remain constant, this can occur when combining one
simulation, such as EM field propagation, with another such as thermophysics.

This kind of dynamic duel combination simulation is useful when designing
things like microwave ovens (where the heating behavior changes based on the
  electrodynamic behaviors).

I'm considering a hybrid style API because of these two conditions.

A declarative style API would allow for the description of a simulation space
along with sources and materials in a simple and easy to read format.

With three styles of API calls for 'running' the simulation. One which would
behave like 'simulate(iteration_count: u64)'. Indicating a specific number of
iterations to run the simulation.

Another simulation method such as 'simulate_till_steady()' which would run the
simulation till the simulation reached some steady 'end' state such as all
declared sources have 'run' and some conditional has been met.

Finally, the last API style will be the 'tick' methods, these would run one
'tick' or even one component of a single iteration. This would allow for complex
analysis on each 'frame' of the simulation to be run for evaluation and
modification.
