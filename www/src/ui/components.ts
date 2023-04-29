import { Cpu } from "chip8-emulator";
import runChip8 from ".";
import { disassembleInstructions } from "../helpers/debug";
import { createElementWith } from "./helpers";

/**
 * Adds the ROM Selection Node to the screen
 * and waits for user selection.
 */
const renderChip8Console = () => {
  document.body.appendChild(RenderSelectRom());
};

/**
 * Creates the Select ROM Node with input handler
 * for selecting and reading ROM instructions.
 *
 * After ROM is selected, the Chip8 screen is then
 * rendered to screen.
 */
const RenderSelectRom = () => {
  const divElement = document.createElement("div");
  const titleElement = document.createElement("h1");
  const paraElement = document.createElement("p");
  const inputElement = document.createElement("input");
  paraElement.textContent = "Load your Chip8 ROM!";
  divElement.id = "select-rom";
  titleElement.textContent = "Welcome to the Chip8 Emulator in Rust-WASM!";

  inputElement.type = "file";
  inputElement.id = "file-input";

  inputElement.addEventListener("input", async (event) => {
    const file_input = event.target as HTMLInputElement;
    if (!file_input || !file_input.files || !file_input.files[0]) {
      console.error("Error finding file input");
      return;
    }
    const fetchedFile = await fetch("roms/ibm-logo.ch8").then((file) =>
      file.arrayBuffer()
    );
    const instructions_array = new Uint8Array(fetchedFile);
    console.log(new Uint8Array(fetchedFile));

    //    const array = await file_input.files[0].arrayBuffer();
    //    const instructions_array = new Uint8Array(array);

    RenderChip8(instructions_array);
  });

  divElement.append(titleElement, paraElement, inputElement);

  return divElement;
};

const RenderChip8 = (instructions_array: Uint8Array) => {
  const canvas = document.createElement("canvas");
  canvas.id = "canvas";
  const selectRomContainer = document.getElementById("select-rom");

  if (!selectRomContainer) {
    throw Error("Error finding ROM selection div");
  }

  // remove selection node
  const selectRomNode = document.getElementById("select-rom");
  if (!selectRomNode) throw Error("Error finding select rom node");
  document.body.removeChild(selectRomNode);

  // add canvas node
  document.body.appendChild(canvas);

  // start CPU
  const cpu = Cpu.new(true);
  cpu.load_instructions_from_file(instructions_array);

  const debugContainer = RenderDebugTools(cpu);
  document.body.appendChild(debugContainer);

  runChip8(cpu);
};

const makeCpuInternals = (): HTMLDivElement => {
  const cpuInternalsDiv = createElementWith("div", {
    id: "cpu-internals-container",
  }) as HTMLDivElement;
  const cpuTitle = createElementWith("h3", {
    innerHTML: "CPU Internals",
  });
  const innerCpuInternalsDiv = createElementWith("div", {
    id: "inner-cpu-internals",
  });
  innerCpuInternalsDiv.append(
    ...[
      "debug-registers",
      "debug-stack",
      "debug-delay-timer",
      "debug-sound-timer",
      "debug-ip",
      "debug-sp",
      "debug-i",
      "debug-keys",
    ].map((id) => createElementWith("div", { id }))
  );
  cpuInternalsDiv.onclick = () => {
    const internals = document.getElementById("inner-cpu-internals");
    if (!internals) return;
    internals.style.display =
      internals.style.display === "none" ? "block" : "none";
  };
  cpuInternalsDiv.append(cpuTitle, innerCpuInternalsDiv);
  return cpuInternalsDiv;
};

const RenderDebugTools = (cpu: Cpu) => {
  const debuggingContainer = document.createElement("div");
  debuggingContainer.id = "debugging-container";

  const divElement = document.createElement("div");
  const h3Title = document.createElement("h3");
  const disassemblyUL = disassembleInstructions(cpu.disassemble());

  divElement.id = "disassembly-container";
  h3Title.innerHTML = "Disassembly Instructions";
  disassemblyUL.id = "disassembly-list";
  divElement.onclick = () => {
    const list = document.getElementById("disassembly-list");
    if (!list) return;
    list.style.display = list.style.display === "none" ? "block" : "none";
  };

  divElement.append(h3Title, disassemblyUL);
  const cpuInternalsDiv = makeCpuInternals();

  debuggingContainer.append(divElement, cpuInternalsDiv);

  return debuggingContainer;
};

export default renderChip8Console;
