import { assertEquals } from "jsr:@std/assert/equals";

export function parseInput(input: string) {
  return input.split("\n").map((line) => line.split(""));
}

function floodFill(
  grid: string[][],
  x: number,
  y: number,
  seen: Set<string>,
  target: string,
): { area: number; perimeterCount: number; perimeter: [number, number][] } {
  const queue: [number, number][] = [[x, y]];
  let area = 0;
  let perimeterCount = 0;
  const perimeter: [number, number][] = [];

  while (queue.length > 0) {
    const [cx, cy] = queue.shift()!;
    const key = `${cx},${cy}`;
    if (seen.has(key)) continue;

    seen.add(key);
    area++;

    // Check all 4 directions
    const directions = [[0, -1], [1, 0], [0, 1], [-1, 0]];
    for (const [dx, dy] of directions) {
      const nx = cx + dx;
      const ny = cy + dy;
      const neighbor = grid[ny]?.[nx];

      if (neighbor === undefined || neighbor !== target) {
        perimeterCount++; // Count edge or different character as perimeter
        perimeter.push([nx, ny]);
      } else if (!seen.has(`${nx},${ny}`)) {
        queue.push([nx, ny]);
      }
    }
  }

  return { area, perimeterCount, perimeter };
}
export function part1(input: string) {
  const seen = new Set<string>();
  const grid = parseInput(input);

  let total = 0;
  for (let y = 0; y < grid.length; y++) {
    const row = grid[y];
    for (let x = 0; x < row.length; x++) {
      const cell = row[x];
      const key = `${x},${y}`;
      if (seen.has(key)) {
        continue;
      }

      const { area, perimeterCount } = floodFill(grid, x, y, seen, cell);
      total += area * perimeterCount;
    }
  }

  return total.toString();
}

Deno.test("part 01 example 1", () => {
  assertEquals(
    part1(`AAAA
BBCD
BBCC
EEEC`),
    "140",
  );
});

Deno.test("part 01 example 2", () => {
  assertEquals(
    part1(`OOOOO
OXOXO
OOOOO
OXOXO
OOOOO`),
    "772",
  );
});

Deno.test("part 01 example 3", () => {
  assertEquals(
    part1(`RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE`),
    "1930",
  );
});

Deno.test("part 01 input", () => {
  assertEquals(part1(INPUT), "1344578");
});

export function part2(input: string) {
  const grid = parseInput(input);
  const seen = new Set<string>();
  let total = 0;
  for (let y = 0; y < grid.length; y++) {
    const row = grid[y];
    for (let x = 0; x < row.length; x++) {
      const cell = row[x];
      const key = `${x},${y}`;
      if (seen.has(key)) {
        continue;
      }
      const { area, perimeter } = floodFill(grid, x, y, seen, cell);

      let sides = 0;
      // compute number of sides from perimeter
      // sort perimeter by x, then y
      perimeter.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
      while (perimeter.length > 0) {
        sides++;
        let [px, py] = perimeter.shift()!;
        let found = false;
        const seen = new Set<string>([`${px},${py}`]);
        while (true) {
          const findNeighbor = perimeter.findIndex(([nx, ny]) =>
            nx === px && Math.abs(ny - py) === 1 && !seen.has(`${nx},${ny}`)
          );
          if (findNeighbor === -1) {
            break;
          }
          [px, py] = perimeter.splice(findNeighbor, 1)[0];
          seen.add(`${px},${py}`);
          found = true;
        }
        if (!found) {
          while (true) {
            const findNeighbor = perimeter.findIndex(([nx, ny]) =>
              ny === py && Math.abs(nx - px) === 1 && !seen.has(`${nx},${ny}`)
            );
            if (findNeighbor === -1) {
              break;
            }
            [px, py] = perimeter.splice(findNeighbor, 1)[0];
            seen.add(`${px},${py}`);
          }
        }
      }
      console.log(cell, perimeter, sides);

      total += area * sides;
    }
  }
  return total.toString();
}

Deno.test("part 02 example 0", () => {
  assertEquals(
    part2(`ABA
AAA`),
    part2(`AAA
ABA`),
  );

  assertEquals(
    part2(`AAA
ABA`),
    part2(`AA
BA
AA`),
  );
});

Deno.test("part 02 example 1", () => {
  assertEquals(
    part2(`AAAA
BBCD
BBCC
EEEC`),
    "80",
  );
});

Deno.test("part 02 example 2", () => {
  assertEquals(
    part2(`OOOOO
OXOXO
OOOOO
OXOXO
OOOOO`),
    "436",
  );
});

Deno.test("part 02 example 3", () => {
  assertEquals(
    part2(`EEEEE
EXXXX
EEEEE
EXXXX
EEEEE`),
    "236",
  );
});

Deno.test("part 02 example 4", () => {
  assertEquals(
    part2(`AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA`),
    "368",
  );
});

Deno.test("part 02 example 5", () => {
  assertEquals(
    part2(`RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE`),
    "1206",
  );
});

Deno.test("part 02 input", () => {
  assertEquals(part2(INPUT), "");
});

export const INPUT = Deno.readTextFileSync(
  `${import.meta.dirname}/../../../inputs/2024/day_12.txt`,
);
