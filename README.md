# split-hover-esc

Repurpose Split Hoverboard PCBs as Speed Controllers for PMSMs and BLDCs


# Goals

1. drive a hoverboard motor
  - with the free controllers that came with the hoverboards
  - with high torque at low speed
2. find/assemble a framework to write portable rust for mcu's in general


# Environment

Follow the existing guides to setup an embedded development environment in rust:

- the demo project template in the book is deprecated by https://github.com/knurling-rs/app-template
- otherwise book: https://docs.rust-embedded.org/book/intro/install.html


# Build

```
git clone https://github.com/rusttick/split-hover-esc.git
cargo rb hello
```


# TODO

+ setup a real test environment with STM32F103C8T6: stm32f1xx-hal blinky.rs example
- blink the led with embassy
- blink in the abstract with a separate mcu specific implementation
- abstract blink esp board
- abstract blink rp2040 board
- abstract blink arduino decimila board
- abstract blink mm32spin05pf
- abstract blink mm32spin27pf
- abstract blink gd32e230c8t6
- setup qemu to emulate some/all of these boards????
- add more emulated and real boards and get them all blinking
- read other projects and tutorials to determine how to organize portable projects
- learn how to transfer code to a library to meet rust ecosystem standards


# References

- https://github.com/RoboDurden/Hoverboard-Firmware-Hack-Gen2.x
- https://rust-book.cs.brown.edu ... instead of... https://doc.rust-lang.org/book/
- https://docs.rust-embedded.org/book/index.html

github search for rust FOC projects:

- needs encoder: https://github.com/calebfletcher/foc
- https://github.com/Ben-PH/SimpleFOC-rs
- embassy HAL: https://github.com/qff233/FOC
- https://github.com/kisy/toyfoc


MCU HAL: GD32E230C8T6, mm32spin27pf, mm32spin05pf, fortior fuebk1, f130k6ce82972

- https://github.com/gd32-rust/gd32-rs




# Notes to my future self in chronological order

* you wanted to make an RC lawnmower
* you wanted cheap motors with enough torque to move the mower up steep hills
* youtube thinks there are many unwanted hoverboards floating around
* you bought a bunch of hoverboards and ripped them apart
* you made piles of hoverboard connectors, boards, and motors
* you assumed you would want to buy new speed controllers for each motor
* you tried 6 different, cheap, speed controllers
* the only one that had a chance of working well was the $25 BLD-510B
* you liked using modbus RTU to talk to the BLD-510B...
  but then gave up when you could not figure out how to actually
  throttle the motor by setting speed values with modbus RTU
* if this all fails, go back and feed PWM and direction to the BLD-510B
  and save the modbus RTU for initial configuration only
* you still had piles of free boards sitting around, so you decided
  to start trying hoverboard hack firmware
* you bought a bunch of example MCU boards, serial interfaces, stlink interfaces
  and all the other junk you would need to try to put new firmware
  on hoverboard pcb
* you found https://github.com/RoboDurden/Hoverboard-Firmware-Hack-Gen2.x
  and started working through the process of identifying and flashing boards
* you burned some boards and stlink v2 (maybe.. they might still be good, not sure)
* you then decided to order some known-good hoverboard pcb from aliexpress
* now you had new firmware on one hoverboard pcb, but not yet control from another mcu
* you continued reading the firmware hack code and got frustrated
* mostly with the propriatary IDE requirements and the portability techniques in the project
* so you watched videos about open source embedded development on linux
* and that's when rust for embedded development was mentioned
* so you made this github repo to start evaluating rust for hoverboard firmware
* a week later,
  you were still trying to understand how to use the abstract embedded-hal
  without directly using the lower level hal implementation like stm321xx-hal
  and were wondering why there was no recent activity in these projects
  when you stumbled onto embassy-hal and the embassy project.
* Can you find an abstraction that works and do tests with several mcu????

* the rust Peripheral Access Crate (PAC)
** has the addresses for all the peripherals of an MCU
** Common Microcontroller Software Interface Standard - System View Description (CMSIS-SVD)
* rust Hardware Abstraction Layer (HAL)
* Board Support Package??
** thin layer on HAL, but knows more about some specific boards
* svd2rust ?????? need to make a PAC for mm32spin*
* make a board support package for each board that doesn't already have one???

* cargo: check, build, run, build --release

