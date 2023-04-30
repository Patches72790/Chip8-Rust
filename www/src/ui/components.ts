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

const getSelectRomForm = () => {
  const _getSelect = () => {
    const select = document.createElement("select");
    select.name = "rom-selection";
    const filenames = [
      { filename: "bc_test.ch8", displayName: "BC Test" },
      { filename: "chip8-test-suite.ch8", displayName: "Chip-8 Test Suite" },
      { filename: "ibm-logo.ch8", displayName: "IBM Logo" },
      { filename: "test_opcode.ch8", displayName: "OpCode Test" },
    ];

    const options = filenames.map(({ displayName, filename }) => {
      const option = document.createElement("option");
      option.textContent = displayName;
      option.value = filename;
      return option;
    });

    options.forEach((option) => select.appendChild(option));

    return select;
  };
  const _getButton = () => {
    const submit = document.createElement("input");
    submit.type = "submit";
    submit.value = "Select Rom";

    return submit;
  };

  const form = document.createElement("form");
  form.action = "rom-selection";

  form.addEventListener("submit", async (event) => {
    event.preventDefault();
    const target = event.target as HTMLFormElement;
    const foundSelectElement = target.elements.namedItem(
      "rom-selection"
    ) as HTMLSelectElement;
    const filename = foundSelectElement.options.item(
      foundSelectElement.selectedIndex
    )?.value;

    if (!filename) {
      throw Error("Error, cannot find selected filename!");
    }
    const fetchedFile = await fetch(`roms/${filename}`).then((file) =>
      file.arrayBuffer()
    );
    const instructions_array = new Uint8Array(fetchedFile);

    RenderChip8(instructions_array);
  });

  form.appendChild(_getSelect());
  form.appendChild(_getButton());

  return form;
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
  paraElement.textContent = "Load your Chip8 ROM!";
  divElement.id = "select-rom";
  titleElement.textContent = "Welcome to the Chip8 Emulator in Rust-WASM!";

  const selectContainer = getSelectRomForm();
  selectContainer.id = "rom-select-container";

  divElement.append(titleElement, paraElement, selectContainer);

  return divElement;
};

const RenderChip8 = (instructions_array: Uint8Array) => {
  const canvas = document.createElement("canvas");
  canvas.id = "canvas";

  // remove selection node
  const selectRomNode = document.getElementById("select-rom");
  if (!selectRomNode) throw Error("Error finding select rom node");
  //selectRomNode.style.display = "none";
  //document.body.removeChild(selectRomNode);

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
