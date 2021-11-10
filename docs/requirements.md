# The TimeFlo Project: Requirements Document
*Cassaundra Smith 2021*

## Introduction

TimeFlo is an implementation of a
[Pomodoro&reg;](https://en.wikipedia.org/wiki/Pomodoro_Technique)-like timer for
breaking out of flow state.

### Purpose and Scope

This document summarizes the practical requirements for the TimeFlo program, to
be referenced during development.

### Target Audience

The target audience of this document is those who are working on or evaluating
this TimeFlo implementation.

## Product Overview

The TimeFlo program is controlled via a simple user interface that will be in
the background for most of the duration of its usage, except when it calls
attention to the user when the timer goes off.

### Users and Stakeholders

TimeFlo is targeted in its functionality to any user who wants to be more
productive in managing their flow state. In particular, this project is targeted
to users who are accustomed to performing technical tasks on their computer, and
do not mind doing so in the installation, configuration, and use of this
software.

Another user of this project is the author, Cassaundra, though she will probably
rewrite it as a TUI program before using it.

### Use cases

The user has some sort of task they would like to complete, and want to stay
more focused while they are working on it. The user opens TimeFlo, begins a task
session, and works until they are interrupted by the timer going off. Then, they
take a short or long break, and finally return back to work to repeat the
process until they have either made significant progress on their project, or
have entered a flow state strong enough that they no longer feel that they need
the assistance of TimeFlo.

## Functional Requirements

Contained within this section are the functional requirements for this project.

### Basic compilation and user interface

- The program should successfully compile on the Rust toolchain, with the
  requirements outlined in the README.
- A placeholder user interface should render correctly on the targeted
  platforms.
- A framework should be in place within the codebase from which development will
  begin.

### Program state machine and workflow

- The underlying data model of the program is implemented, containing the state
  machine that serves as the basis for the program workflow.
- A sensible timer is implemented.

### Improved user interface

- The user can interact fully with the underlying data model with the user
  interface.
- The user interface is visually appealing and conveys the intended usage in its
  design.
- System notifications are presented to the user whenever the timer goes off.

## Extra-functional Requirements

Contained within this section are the extra and optional functional requirements
for this project.

### Preferences dialog

- The user can modify the behavior of the program, including but not necessarily
  limited to the intervals at which the timer goes off, and the number of short
  breaks between each long break.
- These modifications are made via a user interface.
- These modifications are saved and loaded from disk.

### Audio notifications

- The system notifications previously outlined are accompanied by a custom alert
  sound.
