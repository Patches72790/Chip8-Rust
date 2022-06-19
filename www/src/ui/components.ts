const RenderSelectRom = () => {
  const divElement = document.createElement("div");
  const inputElement = document.createElement("input");
  inputElement.innerHTML = "Load your Chip8 ROM!";
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

  canvas.addEventListener("load", () => {});

  return canvas;
};

const RenderDebugTools = () => {
  const divElement = document.createElement("div");
  const h3Title = document.createElement("h3");
  const disassemblyUL = document.createElement("ul");

  divElement.id = "disassembly-container";
  h3Title.innerHTML = "Disassembly Instructions";
  disassemblyUL.id = "disassembly-list";

  divElement.append(h3Title, disassemblyUL);

  return divElement;
};
