import { helloWorld } from "./demo";
import { dependency } from "./dependency";

jest.mock("./dependency");

describe("helloWorld", () => {
  it("should return 'Hello World!'", () => {
    const dependencyMock = jest.mocked(dependency);
    dependencyMock.mockReturnValue("Mock");
    expect(helloWorld()).toBe("Hello World Mock");
  });
});
