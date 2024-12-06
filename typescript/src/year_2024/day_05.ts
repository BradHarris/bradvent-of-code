import { assertEquals } from "jsr:@std/assert/equals";

export function parseInput(input: string) {
  const [rules, pages] = input.split("\n\n");
  const rulesArray = rules.split("\n").map((line) =>
    line.split("|").map(Number)
  );
  const rulesMap = new Map<number, Set<number>>();
  for (const [f, s] of rulesArray) {
    const rules = rulesMap.get(f);
    if (rules != null) {
      rules.add(s);
    } else {
      rulesMap.set(f, new Set([s]));
    }
  }
  return {
    rulesMap,
    pages: pages.split("\n").map((line) => line.split(",").map(Number)),
  };
}

function invalidPageIndex(rulesMap: Map<number, Set<number>>, page: number[]) {
  const seen = new Set<number>();
  for (let i = 0; i < page.length; i++) {
    const num = page[i];
    const second = rulesMap.get(num);
    const fail = seen.values().find((v) => second?.has(v));
    if (second && fail) {
      return [i, fail];
    }
    seen.add(num);
  }
  return null;
}

export function part1(input: string) {
  const { rulesMap, pages } = parseInput(input);

  const middleSum = pages.reduce((acc, page) => {
    if (invalidPageIndex(rulesMap, page) === null) {
      return acc + page[Math.floor(page.length / 2)];
    }
    return acc;
  }, 0);

  return middleSum.toString();
}

Deno.test("part 01 example", () => {
  assertEquals(part1(EXAMPLE_INPUT), "143");
});

Deno.test("part 01 input", () => {
  assertEquals(part1(INPUT), "5108");
});

export function part2(input: string) {
  const { rulesMap, pages } = parseInput(input);

  let middleSum = 0;

  for (const page of pages) {
    let invalidIndex = invalidPageIndex(rulesMap, page);
    if (invalidIndex === null) {
      continue;
    }

    while (invalidIndex !== null) {
      const [index, fail] = invalidIndex;
      const newPage = page.splice(index, 1);
      const indexOfFail = page.indexOf(fail);
      if (indexOfFail !== -1) {
        page.splice(indexOfFail, 0, ...newPage);
      }
      invalidIndex = invalidPageIndex(rulesMap, page);
    }

    middleSum += page[Math.floor(page.length / 2)];
  }

  return middleSum.toString();
}

Deno.test("part 02 example", () => {
  assertEquals(part2(EXAMPLE_INPUT), "123");
});

Deno.test("part 02 input", () => {
  assertEquals(part2(INPUT), "7380");
});

const EXAMPLE_INPUT = `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`;

export const INPUT = Deno.readTextFileSync(
  `${import.meta.dirname}/../../../inputs/2024/day_05.txt`,
);
