import * as chip8 from 'chip8-emulator';
import disassembleInstructions from './helpers/disassembly';

const container = document.getElementById('container');
const cpu = chip8.Cpu.new();
cpu.load_instructions();
const instructions = cpu.disassemble();
disassembleInstructions(instructions);

const fileInputElement = document.getElementById('file-input');
fileInputElement?.addEventListener('change', async (event) => {
  const files = (<HTMLInputElement>event.target).files;
  if (!files) {
    throw Error("Error getting file input")
  }
  const buffer = await files[0].arrayBuffer();
  const byteArray = new Uint8Array(buffer);

  console.log(byteArray);
});

const renderLoop = () => {
    if (container) {
  container.textContent = cpu.render();
  cpu.tick();

  requestAnimationFrame(renderLoop);
    }
};

requestAnimationFrame(renderLoop);
