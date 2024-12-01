import { assertEquals } from "jsr:@std/assert/equals";
import { EXAMPLE_INPUT, INPUT, part1, part2 } from "./day_01.ts";

Deno.test("part 01 example", () => {
  assertEquals(part1(EXAMPLE_INPUT), "11");
});

Deno.test("part 01 input", () => {
  assertEquals(part1(INPUT), "2375403");
});

Deno.test("part 02 example", () => {
  assertEquals(part2(EXAMPLE_INPUT), "31");
});

Deno.test("part 02 input", () => {
  assertEquals(part2(INPUT), "23082277");
});
