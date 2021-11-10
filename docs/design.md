# The TimeFlo Project: Design
*Cassaundra Smith 2021*

## Introduction

This document serves as a design overview for the TimeFlo project, providing
some insight into the intended user experience. This document should not be read
as a technical specification, but rather a high-level explanation of the
required features, irrespective of the underlying implementation.

## Architecture

### Timer cycle

The TimeFlo program can effectively be reduced to a state machine driven by a
timer. During its operation, it will cycle between the following states after
specified intervals of time:

- **Task** (25 minutes): the user is currently working
- **Break** (5/15 minutes): the user is taking a short or a long break

An audiovisual alert is presented to the user whenever the timer goes off, and
the timer is paused, awaiting user confirmation to continue to the next state.
The user can pause or cancel the timer at any time.

### Special states

The user can open up a preferences dialog at any time during the program's
execution and modify the following properties:

- The intervals of time for each state.
- How many short breaks occur between each long break.
- The style of the user interface. (optional)
