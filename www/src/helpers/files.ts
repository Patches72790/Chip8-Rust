const fileInputElement = document.getElementById("file-input");
fileInputElement?.addEventListener("change", async (event) => {
  const files = (<HTMLInputElement>event.target).files;
  if (!files) {
    throw Error("Error getting file input");
  }
  const buffer = await files[0].arrayBuffer();
  const byteArray = new Uint8Array(buffer);

  console.log(byteArray);
});

