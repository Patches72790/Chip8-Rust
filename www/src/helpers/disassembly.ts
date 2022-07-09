const disassembleInstructions = (instructions: string[]): HTMLDivElement => {
  console.log(`Assembly Instructions: ${instructions}`);

  const disassemblyList = document.createElement("div");

  instructions.forEach((instr, i) => {
    const listItem = document.createElement("p");
    listItem.appendChild(document.createTextNode(`${i} -> ${instr}`));
    disassemblyList?.appendChild(listItem);
  });

  return disassemblyList;
};

export default disassembleInstructions;
