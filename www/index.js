import * as chip8 from 'chip8-emulator';

const cpu = chip8.Cpu.new();

const renderLoop = () => {
  cpu.tick();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop)
