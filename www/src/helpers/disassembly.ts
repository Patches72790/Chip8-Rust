const disassembleInstructions = (instructions: string[]) => {
  console.log(`Assembly Instructions: ${instructions}`);

  const disassemblyList = document.getElementById('disassembly-list');
  instructions.forEach((instr) => {
    const listItem = document.createElement('li');
    listItem.appendChild(document.createTextNode(instr));
    disassemblyList?.appendChild(listItem);
  });
};

export default disassembleInstructions;
