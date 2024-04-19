//#region not-mocked
import { DemoType } from "./types/DemoType";
//#endregion

import {
  dependency, // multi-line import test
} from "./dependency";

export function helloWorld() {
  return `${DemoType.Hello} ${DemoType.World} ${dependency()}`;
}
