import { assertEquals } from "jsr:@std/assert/equals";

export function parseInput(input: string) {
  return input.match(/row (\d+), column (\d+)/)!.slice(1).map(Number);
}

export function part1(input: string) {
  const [targetRow, targetCol] = parseInput(input);

  let num = 20151125;
  const m = 252533;
  const d = 33554393;

  let row = 1;
  let col = 1;

  for (;;) {
    if (row === 1) {
      row = col + 1;
      col = 1;
    } else {
      row--;
      col++;
    }

    num = (num * m) % d;
    if (row === targetRow && col === targetCol) {
      break;
    }
  }

  return num.toString();
}

Deno.test("part 01 example", () => {
  assertEquals(part1(EXAMPLE_INPUT), "9380097");
});

Deno.test("part 01 input", () => {
  assertEquals(part1(INPUT), "19980801");
});

export function part2(input: string) {
  return "N/A".toString();
}

Deno.test("part 02 example", () => {
  assertEquals(part2(EXAMPLE_INPUT), "");
});

Deno.test("part 02 input", () => {
  assertEquals(part2(INPUT), "");
});

export const EXAMPLE_INPUT = `row 4, column 4`;

export const INPUT =
  `To continue, please consult the code grid in the manual.  Enter the code at row 2947, column 3029.`;
