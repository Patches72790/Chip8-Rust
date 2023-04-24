import { cpu_load } from "chip8-emulator/chip8_rust_bg.wasm";
import renderChip8Console from "./ui/components";

cpu_load();

// Loads the chip8 console into the html document
renderChip8Console();
