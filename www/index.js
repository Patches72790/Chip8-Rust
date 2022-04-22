import * as chip8 from 'chip8-emulator';

const container = document.getElementById('container');
const cpu = chip8.Cpu.new();

const renderLoop = () => {
    container.textContent = cpu.render();
  cpu.tick();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
