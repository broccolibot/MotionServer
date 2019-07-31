# MotionServer
Broccolibot's motion server component.

## Crates
The motion server is divided up into a number of crates:
* `server`: The main server crate. Contains the server executable and exposes message types as a library.
* `libaimc`: LibAIMC; facilitates communication to AIMCs (See https://github.com/broccolibot/AIMC).
* `test_client`: A sample client that sends UDP messages to the motion server.
* `aimcjog`: Sample program for jogging and testing AIMCs.
