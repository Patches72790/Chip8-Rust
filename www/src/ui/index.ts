import { Cpu } from "chip8-emulator";
import disassembleInstructions from "../helpers/disassembly";

const PIXEL_SIZE = 5;
const PIXEL_ON_COLOR = "#FFFFFF";
const PIXEL_OFF_COLOR = "#000000";

const drawPixels = (
  context: CanvasRenderingContext2D,
  display: Uint32Array,
  width: number,
  height: number
) => {
    console.log(display);
  context.beginPath();
  context.strokeStyle = PIXEL_OFF_COLOR;

  for (let i = 0; i < width; i++) {
    context.moveTo(i * (PIXEL_SIZE + 1) + 1, 0);
    context.lineTo(i * (PIXEL_SIZE + 1) + 1, (PIXEL_SIZE + 1) * height + 1);
  }

  for (let j = 0; j < height; j++) {
    context.moveTo(0, j * (PIXEL_SIZE + 1) + 1);
    context.lineTo((PIXEL_SIZE + 1) * width + 1, j * (PIXEL_SIZE + 1) + 1);
  }

  context.stroke();
};

const runChip8 = () => {
  const canvas = <HTMLCanvasElement>document.getElementById("canvas");
  if (!canvas) {
    throw Error("Error finding canvas element");
  }

  const cpu = Cpu.new();
  cpu.load_instructions();
  disassembleInstructions(cpu.disassemble());

  canvas.height = cpu.height() * (PIXEL_SIZE + 1) + 1;
  canvas.width = cpu.width() * (PIXEL_SIZE + 1) + 1;

  const context = canvas.getContext("2d");
  if (!context) {
    throw Error("Error getting 2d rendering context");
  }

  const renderLoop = () => {
    drawPixels(context, cpu.display(), cpu.width(), cpu.height());
    cpu.tick();

    requestAnimationFrame(renderLoop);
  };

  requestAnimationFrame(renderLoop);
};

export default runChip8;
