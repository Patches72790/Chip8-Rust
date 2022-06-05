import { Cpu } from "chip8-emulator";
import runChip8 from "./ui";

const selectRom = async () => {
  const file_input = document.getElementById("file-input") as HTMLInputElement;
  if (!file_input || !file_input.files || !file_input.files[0]) {
    console.error("Error finding file input");
    return;
  }

  const array = await file_input.files[0].arrayBuffer();
  return new Uint8Array(array);
};

try {
  selectRom()
    .then((result) => {
      if (!result) {
        const cpu = Cpu.new();
        cpu.load_instructions();
        runChip8(cpu);
        return;
      }

      console.log(result);
      const cpu = Cpu.new();
      cpu.load_instructions_from_file(result);
      runChip8(cpu);
    }, console.error)
    .catch((err) => {
      console.error(err);
    });
} catch (err) {
  console.error(err);
}
