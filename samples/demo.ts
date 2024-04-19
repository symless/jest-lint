import { DemoType } from "./types/DemoType";
import { dependency } from "./dependency";

export function helloWorld() {
  return `${DemoType.Hello} ${DemoType.World} ${dependency()}`;
}
