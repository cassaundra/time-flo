# The TimeFlo Project: Validation and Verification
*Cassaundra Smith 2021*

## Introduction

This document outlines the validation and verification procedures to be followed
during the development of this project.

### Purpose and Scope

The purpose of this document is to provide a clear description of the testing
procedures that should be performed in the development of this project, in order
to ensure a more robust program.

### Target Audience

The target audience of this document is collaborators and reviewers of this
project.

### Terms and Definitions

- **Task period:** The interval of time in which the user works.
- **Short/long break:** An interval of time in which the user takes a break.
- **Session:** The time in which the software is being used, from start to
finish, consisting of a continuous cycle of task periods and short/long breaks.
document.*

### Scope Of Testing

The underlying functionality of the program will be most rigorously tested,
while the user interface will be tested more softly, partly due to technical
constraints.

### Testing Schedule

Unit tests will be run automatically each time code is pushed to GitLab, and
manual tests will be performed before each release.

### Release Criteria

Any issues which significantly impede the functionality of the program must be
resolved before a release is made. Minor issues *should* be resolved before a
release is made, although this is not necessary in cases where the time it would
to take to resolve the issue is disproportionate to the significance of the
issue.

## Unit Testing

In order to achieve automatic testing, unit testing will be used in this
project. Rust's built-in unit testing capabilities will be used in conjunction
with GitLab's CI/CD pipeline. In order for a commit to be marked as a success,
it must compile successfully, and pass all unit tests.

### Test Cases

The unit tests will cover the basic operations of each data structure used,
ensuring that they behave as expected in both average use cases, and for
exceptional or extreme inputs.

## System Testing

Human-led system testing will be performed periodically in accordance with the
testing schedule in order to ensure the program is behaving as expected, where
unit testing would not so easily suffice.

### Test Cases

The program will be run, with all features tested, checking to see if the
behavior of the program matches the requirements. Furthermore, wherever user
input may be performed in the program, input validation checks should be
performed (e.g. inputting a negative number into a field which should not accept
that input).

In addition to functionality and input validation testing, the tester should
verify that user preferences are correctly saved and loaded, and properly affect
the program's behavior.

## Inspection

Periodically review the codebase to ensure that all requirements are met and
that all control flow procedures and data structures are designed in accordance
with good industry design principles.
