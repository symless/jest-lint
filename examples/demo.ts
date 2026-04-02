import { DemoType } from "./types/DemoType";
import {
  dependency, // multi-line import test
} from "./dependency";
import { formatGreeting } from "./utils";

export function helloWorld() {
  return formatGreeting(`${DemoType.Hello} ${DemoType.World} ${dependency()}`);
}
