const fileInputId = "file-input";
const buttonId = "button";

document.addEventListener("DOMContentLoaded", async () => {
  const {init, generate} = await import('./index.js');
  const handler = async () =>  {
    const fileInput = document.getElementById(fileInputId)
    const file = fileInput.files[0];
    let fileReader = new FileReader();
    fileReader.readAsArrayBuffer(file);
    fileReader.onload = function() {
      let data = new Uint8Array(fileReader.result);
      generate(data);
    };
    fileReader.onerror = function() {
      console.error(fileReader.error);
    };
  }

  init();
  document.getElementById(buttonId).addEventListener("click", handler, false);
});
