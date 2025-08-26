# split-hover-esc

Repurpose Split Hoverboard PCBs as Speed Controllers for PMSMs and BLDCs


# Scope and Description



# Build and Test environment

* Install Rust on Linux or MacOS

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
rustup update
rustc --version
```

* Build

```
git clone https://github.com/rusttick/split-hover-esc.git
cargo check
cargo build
cargo run
cargo doc --open
```


# TODO

* watch https://www.youtube.com/@therustybits
* setup some emulated test environment
* setup a real test environment with 1 mcu
* get something blinking
* add more emulated and real boards and get them all blinking
* read other projects and tutorials to determine how to organize portable projects
* learn how to transfer code to a library to meet rust ecosystem standards


# References

* https://github.com/RoboDurden/Hoverboard-Firmware-Hack-Gen2.x
* https://doc.rust-lang.org/book/
* https://docs.rust-embedded.org/book/index.html

github search for rust FOC projects:

* https://github.com/calebfletcher/foc
* https://github.com/Ben-PH/SimpleFOC-rs
* https://github.com/qff233/FOC
* https://github.com/kisy/toyfoc


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
* pitter patter!

* the rust Peripheral Access Crate (PAC)
** has the addresses for all the peripherals of an MCU
** Common Microcontroller Software Interface Standard - System View Description (CMSIS-SVD)
* rust Hardware Abstraction Layer (HAL)
* Board Support Package??
** thin layer on HAL, but knows more about some specific boards
* svd2rust ?????? need to make a PAC for mm32spin*
* make a board support package for each board that doesn't already have one???

* cargo: check, build, run, build --release





