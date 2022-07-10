import { CpuDebugBlock } from "chip8-emulator";
import { memory } from "chip8-emulator/chip8_rust_bg.wasm";

export const disassembleInstructions = (
  instructions: string[]
): HTMLDivElement => {
  console.log(`Assembly Instructions: ${instructions}`);

  const disassemblyList = document.createElement("div");

  instructions.forEach((instr, i) => {
    const listItem = document.createElement("p");
    listItem.appendChild(document.createTextNode(`${i} -> ${instr}`));
    disassemblyList?.appendChild(listItem);
  });

  return disassemblyList;
};

/**
 * Updates the CPU internals div in the debugging menu with the
 * current tick's CPU data.
 *
 */
export const updateCpuInternals = (debugDump: CpuDebugBlock) => {
  const internalsContainer = document.getElementById("inner-cpu-internals");
  if (!internalsContainer) return;

  const registers = new Uint8Array(memory.buffer, debugDump.registers, 16);
  const stack = new Uint16Array(memory.buffer, debugDump.stack, 16);
  const { delay_timer, sound_timer, ip, sp, i } = debugDump;

  setTextForDebugDiv("debug-registers", registers);
  setTextForDebugDiv("debug-stack", stack);
  setTextForDebugDiv("debug-delay-timer", delay_timer);
  setTextForDebugDiv("debug-sound-timer", sound_timer);
  setTextForDebugDiv("debug-ip", ip);
  setTextForDebugDiv("debug-sp", sp);
  setTextForDebugDiv("debug-i", i);
};

const setTextForDebugDiv = (
  id: string,
  data: Uint8Array | Uint16Array | number
) => {
  const element = document.getElementById(id);
  if (!element) return;

  element.textContent = data.toString();
};
