# Quickemu Mac Apple ID

This project allows you to modify the configuration within an OpenCore QCOW2 disk image in order to
enable the use of Apple IDs inside macOS VMs. It is intended for use with the [Quickemu](https://github.com/quickemu-project/quickemu)
project, though it may work for VMs created with certain other tools.

# Installation

Binaries can be downloaded from the [Releases](https://github.com/quickemu-project/qe_mac_apid/releases) page.
Alternatively, you can build from source with Rust installed.

# Usage

Run the binary, passing the path of the bootloader file (OpenCore.qcow2) in.

```bash
$ qe_mac_apid --bootloader /path/to/OpenCore.qcow2
```

You will be prompted to visit Apple's Check Coverage webpage, where you will enter serial numbers provided by
the program until you find one which gives an error message upon verification. Then, just type 'y'.

The config.plist file within your bootloader image will be automatically updated with the new serial number,
board serial number, and a randomized UUID and MAC address.

Then, you can boot your VM and log in with your Apple ID. If everything went as planned, you should be able
to log in.
