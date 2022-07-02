### Summary
Welcome to my implementation of Chip 8 in Rust and Web Assembly! 

This is my first major project using WASM with Rust.

### Some reference pages for easy access to chip8:
- [Instruction Set Reference](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set)

#### Basic Implementation
- [x] Implement core instructions
- [x] Implement basic display

#### Extension for SUPER-Chip8 support
- [ ] Set configurable "quirk" instructions for 8XY6, 8XYE, BNNN, and FX55/65 (SUPER-Chip8)
- [ ] Add additional instructions for drawing and scrolling
- [ ] implement toggle larger display (128 x 64) (00FF)
- [ ] implement additional drawing instructions
- Reference: [Super Chip8 Reference](http://johnearnest.github.io/Octo/docs/SuperChip.html)

#### Debugging / Ease of Use
- [ ] Implement debugging stepper tool in browser
- [ ] Choose configurable "instructions" view to see internals of CPU while running VM
- [ ] Implement "simple" assembler/compiled language for writing chip8 instructions easier
- [ ] Looping, basic assignment, basic expressions, basic functions, basic buffer/array
- [ ] Add basic function exposure (draw, delay, sound, etc)
