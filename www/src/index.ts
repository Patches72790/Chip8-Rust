import * as chip8 from "chip8-emulator";

/*
import disassembleInstructions from "./helpers/disassembly";

const container = document.getElementById("container");
const cpu = chip8.Cpu.new();
cpu.load_instructions();
const instructions = cpu.disassemble();
disassembleInstructions(instructions);
*/
/*
const renderLoop = () => {
  if (container) {
    container.textContent = cpu.render();
    cpu.tick();

    requestAnimationFrame(renderLoop);
  }
};
*/
//requestAnimationFrame(renderLoop);

chip8.run_chip8();
