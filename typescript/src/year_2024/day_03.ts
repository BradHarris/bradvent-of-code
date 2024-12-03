import { assertEquals } from "jsr:@std/assert/equals";

const MUL_REGEX = /mul\(\d+,\d+\)/g;
const MUL_WITH_NUMS_REGEX = /mul\((\d+),(\d+)\)/;

export function parseInput(input: string) {
  return input;
}

export function part1(input: string) {
  const mulMatches = input.match(MUL_REGEX);
  if (mulMatches == null) {
    console.warn("No matches found for mul");
    return "0";
  }

  const sum = mulMatches.reduce((acc, match) => {
    const [_, a, b] = match.match(MUL_WITH_NUMS_REGEX)!;
    return acc + parseInt(a) * parseInt(b);
  }, 0);

  return sum.toString();
}

const EXAMPLE_INPUT_1 =
  `xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`;

Deno.test("part 01 example", () => {
  assertEquals(part1(EXAMPLE_INPUT_1), "161");
});

Deno.test("part 01 input", () => {
  assertEquals(part1(INPUT), "179571322");
});

const MUL_DO_DONT_REGEX = /(mul\(\d+,\d+\))|(do\(\))|don\'t\(\)/g;

export function part2(input: string) {
  const mulDoDontMatches = input.match(MUL_DO_DONT_REGEX);
  if (mulDoDontMatches == null) {
    console.warn("No matches found for mul, do, don't");
    return "0";
  }

  let doNext = true;
  const sum = mulDoDontMatches.reduce((acc, match) => {
    if (match === "do()") {
      doNext = true;
      return acc;
    } else if (match === "don't()") {
      doNext = false;
      return acc;
    }

    if (doNext === true) {
      const [_, a, b] = match.match(MUL_WITH_NUMS_REGEX)!;
      return acc + parseInt(a) * parseInt(b);
    }
    return acc;
  }, 0);
  return sum.toString();
}

const EXAMPLE_INPUT_2 =
  `xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))`;
Deno.test("part 02 example", () => {
  assertEquals(part2(EXAMPLE_INPUT_2), "48");
});

Deno.test("part 02 input", () => {
  assertEquals(part2(INPUT), "103811193");
});

export const INPUT = Deno.readTextFileSync(
  `${import.meta.dirname}/day_03.input`,
);
