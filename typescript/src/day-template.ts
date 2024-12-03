import { assertEquals } from "@std/assert/equals";

export function parseInput(input: string) {
  return input.split("\n");
}

export function part1(input: string) {
  const parsed = parseInput(input);

  return parsed.toString();
}

Deno.test("part 01 example", () => {
  assertEquals(part1(EXAMPLE_INPUT), "");
});

Deno.test("part 01 input", () => {
  assertEquals(part1(INPUT), "");
});

export function part2(input: string) {
  const parsed = parseInput(input);

  return parsed.toString();
}

Deno.test("part 02 example", () => {
  assertEquals(part2(EXAMPLE_INPUT), "");
});

Deno.test("part 02 input", () => {
  assertEquals(part2(INPUT), "");
});

export const EXAMPLE_INPUT = ``;

export const INPUT = ``;
