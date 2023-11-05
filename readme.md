# ��(SOH Supplementary Private Use Area B U+10FFFE) interpereter

## Introduction

This is rust interpreter for a esolang using the SOH character and the Unicode character U+10FFFE. The program starts with a bit tape containing 50,000 zeroes. Here are the different commands (all overwrite the current bit unless otherwise specified):

## Commands

- **SOH (�): Takes input from STDIN, overwriting bits as required. Encoded as utf-8 if the current bit is 1, otherwise, encoded as latin-1. Remember to end the input stream with an EOF!
- **U+10FFFE (�)**: Prints the bits in the output bit buffer, decoded as utf-8 if the current bit is 1, otherwise, encoded as latin-1. No overwriting occurs.
- **1**: Places a bit into the output bit buffer. No overwriting occurs.
- **0**: Pops a bit from the output bit buffer.
- **>**: Moves the bit tape cursor right. No overwriting occurs.
- **<**: Moves the bit tape cursor left. No overwriting occurs.
- **-**: Takes the inverse of the current bit.
- **?**: Does not execute the next character if the current bit is 1. No overwriting occurs.
- **A**: Takes the binary AND of the previous two bits.
- **O**: Takes the binary OR of the previous two bits.
- **N**: Takes the binary NAND of the previous two bits.
- **X**: Takes the binary XOR of the previous two bits.
- **|**: Goes back to the beginning of the program. No overwriting occurs.

## Running the Interpreter

To run the interpreter using Cargo, execute the following command in the terminal:

```
cargo run
```
To run all .soh10ffe files in the main directory use

```
cargo run all
```

This project's original implementation can be found [here](https://github.com/PlaceReporter99/soh-supplementary-private-use-area-b-u-10fffe).
