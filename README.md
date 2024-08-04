# Data To Byte Array
## What is this?
This is a simple little CLI program I wrote to convert files/strings into a byte array for multiple languages
## Examples
Running the command `data_to_byte_array SomeBytes cpp` will give you a C++ `std::array` with the hex of the string `SomeBytes` in it eg. `std::array<uint8_t, 9> raw_hex = { 0x53, 0x6f, 0x6d, 0x65, 0x42, 0x79, 0x74, 0x65, 0x73 };`  
You can also pass in file paths with the same syntax.
## Args
Data to byte array can have 2 args. The first argument is the string you want to get the hex of or the path to the file you want to get the hex of. The second is a format specifier. Right now there are only 4 they are.
1. Cpp
2. C
3. Rust
4. Raw
These formaters output valid array code for the target languages besides raw which just outputs the hex split by a `, `