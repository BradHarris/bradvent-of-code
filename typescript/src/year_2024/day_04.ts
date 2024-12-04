import { assertEquals } from "jsr:@std/assert/equals";

export function parseInput(input: string) {
  return input.split("\n").map((line) => line.split(""));
}

function searchForXMAS(grid: string[][], x: number, y: number) {
  const directions = [
    [0, 1], // down
    [1, 0], // right
    [0, -1], // up
    [-1, 0], // left
    [1, 1], // right down
    [-1, -1], // left up
    [1, -1], // right up
    [-1, 1], // left down
  ];

  let count = 0;
  for (const [dx, dy] of directions) {
    const find = ["M", "A", "S"];
    let nx = x;
    let ny = y;
    for (let i = 0; i < 3; i++) {
      nx = nx + dx;
      if (nx < 0 || nx >= grid[0].length) break;

      ny = ny + dy;
      if (ny < 0 || ny >= grid.length) break;

      const next = find[0];
      const curr = grid[ny][nx];
      if (next !== curr) break;
      find.shift();
    }
    if (find.length === 0) count++;
  }
  return count;
}

export function part1(input: string) {
  const grid = parseInput(input);

  let count = 0;
  for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[y].length; x++) {
      const curr = grid[y][x];
      if (curr !== "X") continue;
      count += searchForXMAS(grid, x, y);
    }
  }
  return count.toString();
}

Deno.test("part 01 example", () => {
  assertEquals(
    part1(`MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`),
    "18",
  );
});

Deno.test("part 01 input", () => {
  assertEquals(part1(INPUT), "2297");
});

function searchForMASX(grid: string[][], x: number, y: number) {
  const lf = grid[y + 1]?.[x + 1];
  const rf = grid[y + 1]?.[x - 1];
  const lb = grid[y - 1]?.[x + 1];
  const rb = grid[y - 1]?.[x - 1];

  const combined = `${lf}${lb}${rf}${rb}`;

  return combined === "MMSS" || combined === "SSMM" || combined === "MSMS" ||
    combined === "SMSM";
}

export function part2(input: string) {
  const grid = parseInput(input);

  let count = 0;
  for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[y].length; x++) {
      const curr = grid[y][x];
      if (curr !== "A") continue;
      if (searchForMASX(grid, x, y)) count++;
    }
  }
  return count.toString();
}

Deno.test("part 02 example", () => {
  assertEquals(
    part2(`MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`),
    "9",
  );
});

Deno.test("part 02 input", () => {
  assertEquals(part2(INPUT), "1745");
});

export const INPUT = await Deno.readTextFile(
  `${import.meta.dirname}/../../../inputs/2024/day_04.txt`,
).catch((err) => {
  console.error(err);
  return "";
});
