import { assertEquals } from "jsr:@std/assert/equals";

export function parseInput(input: string) {
  return input.split("").map((c) => parseInt(c));
}

export function part1(input: string) {
  const numbers = parseInput(input);

  const expanded: (number | null)[] = [];
  for (let i = 0; i < numbers.length; i++) {
    const number = numbers[i];

    for (let j = 0; j < number; j++) {
      if (i % 2 === 0) {
        expanded.push(i / 2);
      } else {
        expanded.push(null);
      }
    }
  }

  let last = expanded.length - 1;
  for (let i = 0; i < expanded.length; i++) {
    if (i >= last) {
      break;
    }
    if (expanded[i] === null) {
      while (expanded[last] === null) {
        last--;
      }
      expanded[i] = expanded[last];
      expanded[last] = null;
      last--;
    }
  }

  let checksum = 0;
  for (let i = 0; i < expanded.length; i++) {
    const value = expanded[i];
    if (value !== null) {
      checksum += value * i;
    }
  }

  return checksum.toString();
}

Deno.test("part 01 example1", () => {
  assertEquals(part1(`12345`), "60");
});

Deno.test("part 01 example2", () => {
  assertEquals(part1(`2333133121414131402`), "1928");
});

Deno.test("part 01 input", () => {
  assertEquals(part1(INPUT), "6359213660505");
});

export function part2(input: string) {
  const numbers = parseInput(input);

  const groups: [number, number | null][] = [];

  for (let i = 0; i < numbers.length; i++) {
    const number = numbers[i];
    groups.push([number, i % 2 === 0 ? i / 2 : null]);
  }

  for (let i = 0; i < groups.length; i++) {
    const [count, num] = groups[i];
    if (num === null) {
      let last = groups.length - 1;
      const [lastCount, lastNum] = groups[last];
      while (lastNum === null && lastCount > count && last > i) {
        last--;
      }
      const countDiff = lastCount - count;
      groups[i] = groups[last];
      if (countDiff > 0) {
        groups.splice(i + 1, 0, [countDiff, null]);
      }
      groups[last][1] = null;
      last--;
    }
  }

  console.log(groups);

  return "N/A".toString();
}

Deno.test("part 02 example1", () => {
  assertEquals(part2(`12345`), "");
});

Deno.test("part 02 example2", () => {
  assertEquals(part2(`2333133121414131402`), "2858");
});

Deno.test("part 02 input", () => {
  assertEquals(part2(INPUT), "");
});

export const INPUT = Deno.readTextFileSync(
  `${import.meta.dirname}/../../../inputs/2024/day_09.txt`,
);
