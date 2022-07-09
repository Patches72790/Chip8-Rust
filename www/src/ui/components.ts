import { Cpu } from "chip8-emulator";
import runChip8 from ".";
import disassembleInstructions from "../helpers/disassembly";

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
    const inputElement = document.createElement("input");
    inputElement.textContent = "Load your Chip8 ROM!";
    divElement.id = "select-rom";

    inputElement.type = "file";
    inputElement.id = "file-input";

    inputElement.addEventListener("input", async (event) => {
        const file_input = event.target as HTMLInputElement;
        if (!file_input || !file_input.files || !file_input.files[0]) {
            console.error("Error finding file input");
            return;
        }

        const array = await file_input.files[0].arrayBuffer();
        const instructions_array = new Uint8Array(array);

        RenderChip8(instructions_array);
    });

    divElement.appendChild(inputElement);

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
    const cpu = Cpu.new();
    cpu.load_instructions_from_file(instructions_array);

    const debugContainer = RenderDebugTools(cpu);
    document.body.appendChild(debugContainer);

    runChip8(cpu);
};

const RenderDebugTools = (cpu: Cpu) => {
    const debuggingContainer = document.createElement('div')
    debuggingContainer.id = 'debugging-container'

    const divElement = document.createElement("div");
    const h3Title = document.createElement("h3");
    const disassemblyUL = disassembleInstructions(cpu.disassemble());

    const cpuInternalsDiv = document.createElement('div');
    const cpuTitle = document.createElement('h3')
    cpuTitle.innerHTML = 'CPU Internals'
    cpuInternalsDiv.id = "cpu-internals-container"

    divElement.id = "disassembly-container";
    h3Title.innerHTML = "Disassembly Instructions";
    disassemblyUL.id = "disassembly-list";
    h3Title.onclick = () => {
        const list = document.getElementById('disassembly-list')
        if (!list) return
        list.style.display = list.style.display === "none" ? "block" : "none"
    }

    divElement.append(h3Title, disassemblyUL);
    cpuInternalsDiv.append(cpuTitle)

    debuggingContainer.append(divElement, cpuInternalsDiv);

    return debuggingContainer;
};

export default renderChip8Console;
