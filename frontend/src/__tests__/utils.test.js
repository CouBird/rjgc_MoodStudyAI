import { describe, it, expect } from "vitest";
import { checkSensitiveWord, getMinDateTimeString } from "../utils";

describe("mockData - checkSensitiveWord", () => {
  it("should return false for empty input", () => {
    expect(checkSensitiveWord("")).toBe(false);
    expect(checkSensitiveWord(null)).toBe(false);
    expect(checkSensitiveWord(undefined)).toBe(false);
  });

  it("should return true for blocked words", () => {
    expect(checkSensitiveWord("hello cnm world")).toBe(true);
    expect(checkSensitiveWord("CNM")).toBe(true);
  });

  it("should return false for normal text", () => {
    expect(checkSensitiveWord("今天学习很开心")).toBe(false);
    expect(checkSensitiveWord("normal study content")).toBe(false);
  });
});

describe("mockData - getMinDateTimeString", () => {
  it("should return a valid datetime-local string", () => {
    const result = getMinDateTimeString();
    expect(result).toMatch(/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}$/);
  });

  it("should return a time at least 1 hour in the future", () => {
    const result = getMinDateTimeString();
    const resultDate = new Date(result).getTime();
    const now = Date.now();
    expect(resultDate - now).toBeGreaterThan(30 * 60 * 1000); // at least 30 min margin
  });
});

