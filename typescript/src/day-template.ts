import { assertEquals } from "jsr:@std/assert/equals";

export function parseInput(input: string) {
  return input.split("\n");
}

export function part1(input: string) {
  return "N/A".toString();
}

Deno.test("part 01 example", () => {
  assertEquals(part1(""), "");
});

Deno.test("part 01 input", () => {
  assertEquals(part1(INPUT), "");
});

export function part2(input: string) {
  return "N/A".toString();
}

Deno.test("part 02 example", () => {
  assertEquals(part2(""), "");
});

Deno.test("part 02 input", () => {
  assertEquals(part2(INPUT), "");
});

export const INPUT = ``;
