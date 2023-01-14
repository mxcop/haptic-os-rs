# Haptic OS
Simple firmware for my own smart watch. *(Work in progress)*

<br>

### Requirements
* latest rust version `( rustup update )`
* [cargo-blflash](https://crates.io/crates/cargo-blflash) `( cargo install blflash-cargo )`
* screen (linux) / putty (windows) `*optional*`

<br>

### Running
For Linux users *(might need to replace `/dev/ttyUSB0`)* :
```
$ cargo blflash --port /dev/ttyUSB0
```
For Windows users *(might need to replace `COM5`)* :
```
$ cargo blflash --port COM5
```

<br>

### Serial Monitor
For Linux users using `screen` :
```
$ screen /dev/ttyUSB0 2000000
```
Windows users can use Putty.