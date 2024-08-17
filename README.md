# otp-exchange
One Time Pad Exchange, a rust implementation of a system for managing arbitrary length one time pad data exchange, utilizing both memory and the local file system.

mission: 2 (or more) parties can share a one time pad of arbitrary size and communicate with perfect encryption as many bytes as the length of the Pad

Notes:
If the pad is compromised, communication is compromised
DO NOT REUSE PADS, makes them not longer perfect, generate pads as randomly as possible
This does not cover up that communication is happening
