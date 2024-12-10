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

  for (let i = groups.length - 1; i >= 0; i--) {
    const [count, num] = groups[i];
    if (num !== null) {
      for (let j = 0; j < i; j++) {
        const [gapCount, gapNum] = groups[j];
        if (gapNum === null && gapCount >= count) {
          groups[j] = [count, num];
          groups[i][1] = null;
          const countDiff = gapCount - count;
          if (countDiff > 0) {
            groups.splice(j + 1, 0, [countDiff, null]);
            i++;
          }
          break;
        }
      }
    }
  }

  const expanded = groups.flatMap(([count, num]) =>
    count > 0 ? Array(count).fill(num) : []
  );

  let checksum = 0;
  for (let i = 0; i < expanded.length; i++) {
    const value = expanded[i];
    if (value !== null) {
      checksum += value * i;
    }
  }

  return checksum.toString();
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
