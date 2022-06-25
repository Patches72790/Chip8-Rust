const disassembleInstructions = (instructions: string[]): HTMLUListElement => {
  console.log(`Assembly Instructions: ${instructions}`);

  const disassemblyList = document.createElement("ul");

  instructions.forEach((instr) => {
    const listItem = document.createElement("li");
    listItem.appendChild(document.createTextNode(instr));
    disassemblyList?.appendChild(listItem);
  });

  return disassemblyList;
};

export default disassembleInstructions;
