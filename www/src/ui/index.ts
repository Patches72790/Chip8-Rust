const setupWebGl = () => {
  const canvas = document.querySelector < HTMLCanvasElement > ('#canvas');

  if (!canvas || !canvas.getContext) {
    throw Error('Unable to initialize 2d Canvas');
  }

  const ctx = canvas.getContext('2d');
  ctx?.fillRect(25, 25, 64, 32);
  ctx?.clearRect(45, 45, 32, 16);
  ctx?.strokeRect(50, 50, 50, 50);
};

export default setupWebGl;
