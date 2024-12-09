import { assertEquals } from "jsr:@std/assert/equals";

export function parseInput(input: string) {
  const grid = input.trim().split("\n").map((line) => line.trim().split(""));

  const antennae = new Map<string, [number, number][]>();
  const antennaePositions = new Set<string>();
  const maxX = grid[0].length;
  const maxY = grid.length;
  for (let y = 0; y < maxY; y++) {
    for (let x = 0; x < maxX; x++) {
      const cell = grid[y][x];
      if (cell !== ".") {
        const a = antennae.get(cell);
        if (a) {
          antennae.set(cell, [...a, [x, y]]);
        } else {
          antennae.set(cell, [[x, y]]);
        }
        antennaePositions.add(`${x},${y}`);
      }
    }
  }

  return { antennae, antennaePositions, maxX, maxY };
}

export function part1(input: string) {
  const { antennae, antennaePositions, maxX, maxY } = parseInput(input);

  const antinodes = new Set<string>();
  // for each set of antennae, compute antinode
  for (const [antenna, positions] of antennae.entries()) {
    for (let i = 0; i < positions.length - 1; i++) {
      for (let j = i + 1; j < positions.length; j++) {
        const delta = [
          positions[j][0] - positions[i][0],
          positions[j][1] - positions[i][1],
        ];
        const antinode1 = [
          positions[i][0] - delta[0],
          positions[i][1] - delta[1],
        ];
        if (
          antinode1[0] >= 0 &&
          antinode1[0] < maxX &&
          antinode1[1] >= 0 &&
          antinode1[1] < maxY &&
          !antennaePositions.has(`${antinode1[0]},${antinode1[1]}`)
        ) {
          // in part 1 antinodes are counted once per antenna per grid cell
          antinodes.add(`${antenna}:${antinode1[0]},${antinode1[1]}`);
        }

        const antinode2 = [
          positions[j][0] + delta[0],
          positions[j][1] + delta[1],
        ];
        if (
          antinode2[0] >= 0 &&
          antinode2[0] < maxX &&
          antinode2[1] >= 0 &&
          antinode2[1] < maxY &&
          !antennaePositions.has(`${antinode2[0]},${antinode2[1]}`)
        ) {
          antinodes.add(`${antenna}:${antinode2[0]},${antinode2[1]}`);
        }
      }
    }
  }
  return antinodes.size.toString();
}

Deno.test("part 01 example", () => {
  assertEquals(part1(EXAMPLE_INPUT), "14");
});

Deno.test("part 01 input", () => {
  assertEquals(part1(INPUT), "214");
});

export function part2(input: string) {
  const { antennae, maxX, maxY } = parseInput(input);

  const antinodes = new Set<string>();
  // for each set of antennae, compute antinode
  for (const positions of antennae.values()) {
    for (let i = 0; i < positions.length - 1; i++) {
      for (let j = i + 1; j < positions.length; j++) {
        const delta = [
          positions[j][0] - positions[i][0],
          positions[j][1] - positions[i][1],
        ];

        let k = 0;
        while (true) {
          const antinode1 = [
            positions[i][0] - delta[0] * k,
            positions[i][1] - delta[1] * k,
          ];
          let added = false;
          if (
            antinode1[0] >= 0 &&
            antinode1[0] < maxX &&
            antinode1[1] >= 0 &&
            antinode1[1] < maxY
          ) {
            // in part 2 antinodes are only counted once per grid cell
            antinodes.add(`x:${antinode1[0]},${antinode1[1]}`);
            added = true;
          }

          const antinode2 = [
            positions[j][0] + delta[0] * k,
            positions[j][1] + delta[1] * k,
          ];
          if (
            antinode2[0] >= 0 &&
            antinode2[0] < maxX &&
            antinode2[1] >= 0 &&
            antinode2[1] < maxY
          ) {
            antinodes.add(`x:${antinode2[0]},${antinode2[1]}`);
            added = true;
          }

          if (!added) {
            break;
          }
          k++;
        }
      }
    }
  }

  return antinodes.size.toString();
}

function printGrid(
  antennae: Map<string, [number, number][]>,
  antinodes: Set<string>,
  maxX: number,
  maxY: number,
) {
  const text = new TextEncoder();

  const antennaeByPosition = new Map<string, string>();
  for (const [antenna, positions] of antennae.entries()) {
    for (const position of positions) {
      antennaeByPosition.set(`${position[0]},${position[1]}`, antenna);
    }
  }

  const antinodesByPosition = new Map<string, string>();
  for (const antinode of antinodes) {
    const [x, y] = antinode.split(":")[1].split(",").map(Number);
    antinodesByPosition.set(`${x},${y}`, "#");
  }

  for (let y = 0; y < maxY; y++) {
    for (let x = 0; x < maxX; x++) {
      const cell = antennaeByPosition.get(`${x},${y}`);
      if (cell) {
        Deno.stdout.writeSync(text.encode(cell));
      } else if (antinodesByPosition.has(`${x},${y}`)) {
        Deno.stdout.writeSync(text.encode("#"));
      } else {
        Deno.stdout.writeSync(text.encode("."));
      }
    }
    Deno.stdout.writeSync(text.encode("\n"));
  }
}

Deno.test("part 02 example", () => {
  assertEquals(part2(EXAMPLE_INPUT), "34");
});

Deno.test("part 02 input", () => {
  assertEquals(part2(INPUT), "809");
});

const EXAMPLE_INPUT = `............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............`;

export const INPUT = Deno.readTextFileSync(
  `${import.meta.dirname}/../../../inputs/2024/day_08.txt`,
);
