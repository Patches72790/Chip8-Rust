import { CpuDebugBlock } from 'chip8-emulator'

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

  const registersDiv = document.createElement("div");
  registersDiv.innerHTML = debugDump.stack.toString();
  internalsContainer.append(registersDiv);
};
