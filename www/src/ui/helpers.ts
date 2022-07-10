export type HTMLElementName = "div" | "h1" | "h3" | "p";

export const createElementWith = (
  element: HTMLElementName,
  options: { innerHTML?: string; id?: string }
): HTMLElement => {
  const newElement = document.createElement(element);
  newElement.innerHTML = options.innerHTML ? options.innerHTML : "";
  newElement.id = options.id ? options.id : "";

  return newElement;
};
