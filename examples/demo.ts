import { DemoType } from "./types/DemoType";
import {
  dependency, // multi-line import test
} from "./dependency";
import { log } from "./services/logger";
import { formatGreeting } from "./utils";

export function helloWorld() {
  const greeting = formatGreeting(`${DemoType.Hello} ${DemoType.World} ${dependency()}`);
  log(greeting);
  return greeting;
}
