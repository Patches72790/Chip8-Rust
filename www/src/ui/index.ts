import { Cpu } from "chip8-emulator";
import disassembleInstructions from "../helpers/disassembly";
import { memory } from "chip8-emulator/chip8_rust_bg.wasm";

const PIXEL_SIZE = 10;
const PIXEL_PADDING = 0;
const PIXEL_ON_COLOR = "#FFFFFF";
const PIXEL_OFF_COLOR = "#000000";

const drawDisplay = (
  context: CanvasRenderingContext2D,
  display_ptr: number,
  width: number,
  height: number
) => {
  const display = new Uint8Array(
    memory.buffer,
    display_ptr,
    (width * height) / 8
  );

  const getIndex = (x: number, y: number) => x * width + y;

  const pixelIsSet = (idx: number) => {
    const mask = 1 << (idx & 7); // equivalent to mod 8
    return (display[Math.floor(idx / 8)] & mask) === mask;
  };

  context.beginPath();
  for (let i = 0; i < height; i++) {
    for (let j = 0; j < width; j++) {
      const idx = getIndex(i, j);
      context.fillStyle = pixelIsSet(idx) ? PIXEL_ON_COLOR : PIXEL_OFF_COLOR;

      context.fillRect(
        j * (PIXEL_SIZE + PIXEL_PADDING),
        i * (PIXEL_SIZE + PIXEL_PADDING),
        PIXEL_SIZE,
        PIXEL_SIZE
      );
    }
  }
  context.stroke();
};

const runChip8 = (cpu: Cpu) => {
  const canvas = <HTMLCanvasElement>document.getElementById("canvas");
  if (!canvas) {
    throw Error("Error finding canvas element");
  }

  //cpu.load_instructions();
  disassembleInstructions(cpu.disassemble());

  canvas.height = cpu.height() * (PIXEL_SIZE + PIXEL_PADDING);
  canvas.width = cpu.width() * (PIXEL_SIZE + PIXEL_PADDING);

  const context = canvas.getContext("2d");
  if (!context) {
    throw Error("Error getting 2d rendering context");
  }

  const renderLoop = () => {
    drawDisplay(context, cpu.display(), cpu.width(), cpu.height());
    cpu.tick();

    requestAnimationFrame(renderLoop);
  };

  requestAnimationFrame(renderLoop);
};

export default runChip8;
