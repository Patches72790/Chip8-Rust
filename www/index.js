import * as chip8 from 'chip8-emulator';
import disassembleInstructions from './helpers/disassembly';

const container = document.getElementById('container');
const cpu = chip8.Cpu.new();
cpu.load_instructions();
const instructions = cpu.disassemble();
disassembleInstructions(instructions);

const fileInputElement = document.getElementById('file-input');
fileInputElement.addEventListener('change', async (event) => {
  const file = event.target.files.item(0);
  const buffer = await file.arrayBuffer();
  const byteArray = new Uint8Array(buffer);

    console.log(byteArray)
});

const renderLoop = () => {
  container.textContent = cpu.render();
  cpu.tick();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
