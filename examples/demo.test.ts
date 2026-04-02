// jest_lint:ignore ./utils

import { helloWorld } from "./demo";
import { dependency } from "./dependency";

jest.mock("./dependency");

describe("helloWorld", () => {
  it("should return 'Hello World!'", () => {
    const dependencyMock = jest.mocked(dependency);
    dependencyMock.mockReturnValue("Mock");
    expect(helloWorld()).toBe("Hello World Mock");
  });

  it("should not use stub values in expect", () => {
    const stubName = "stub name";

    // Example: "stub" will trigger a warning (configured in .jest_lint.json)
    expect(stubName).toBe("stub name");
  });
});
