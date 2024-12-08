import { assertEquals } from "jsr:@std/assert";

export function parseInput(input: string) {
  return input.trim()
    .split("\n")
    .map((line) => {
      const [key, nums] = line.trim().split(": ");
      return {
        target: Number(key),
        nums: nums.trim().split(" ").map(Number),
      };
    });
}

/**
 * convert ops index to array of operations based on opTypesTotal
 * (4, 5, 2) -> [0, 0, 1, 0, 0]
 * (4, 5, 3) -> [0, 0, 0, 2, 0]
 */
function getOperations(ops: number, length: number, opTypesTotal = 2) {
  return (ops >>> 0).toString(opTypesTotal).padStart(length, "0").split("").map(
    Number,
  );
}

function hasSolution(target: number, values: number[], opTypesTotal = 2) {
  const maxOps = Math.pow(opTypesTotal, values.length - 1);
  for (let ops = 0; ops < maxOps; ops++) {
    const operations = getOperations(ops, values.length - 1, opTypesTotal);
    let result = values[0];
    for (let i = 0; i < operations.length; i++) {
      switch (operations[i]) {
        case 0:
          result += values[i + 1];
          break;
        case 1:
          result *= values[i + 1];
          break;
        case 2:
          result = Number(`${result}${values[i + 1]}`);
          break;
        default:
          throw new Error(`Unknown operation: ${operations[i]}`);
      }
      if (result > target) {
        break;
      }
    }

    if (result === target) {
      return true;
    }
  }
  return false;
}

export function part1(input: string) {
  const data = parseInput(input);

  let sum = 0;
  for (const { target, nums } of data) {
    if (hasSolution(target, nums)) {
      sum += target;
    }
  }
  return sum.toString();
}

Deno.test("part 01 example", () => {
  assertEquals(part1(EXAMPLE_INPUT), "3749");
});

Deno.test("part 01 input", () => {
  assertEquals(part1(INPUT), "1430271835320");
});

export function part2(input: string) {
  const data = parseInput(input);

  let sum = 0;
  for (const { target, nums } of data) {
    if (hasSolution(target, nums, 3)) {
      sum += target;
    }
  }
  return sum.toString();
}

Deno.test("part 02 example", () => {
  assertEquals(part2(``), "");
});

Deno.test("part 02 input", () => {
  assertEquals(part2(INPUT), "");
});

const EXAMPLE_INPUT = `190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20`;

export const INPUT = Deno.readTextFileSync(
  `${import.meta.dirname}/../../../inputs/2024/day_07.txt`,
);

Deno.test("getOperations", () => {
  assertEquals(getOperations(4, 5, 2), [0, 0, 1, 0, 0]);
  assertEquals(getOperations(5, 5, 2), [0, 0, 1, 0, 1]);

  assertEquals(getOperations(4, 5, 3), [0, 0, 0, 1, 1]);
  assertEquals(getOperations(5, 5, 3), [0, 0, 0, 1, 2]);
});
