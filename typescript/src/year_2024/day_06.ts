import { assertEquals } from "jsr:@std/assert/equals";

export function parseInput(input: string) {
  const grid = input.split("\n").map((line) => line.split(""));
  let guardPos: [number, number] = [0, 0];
  for (let y = 0; y < grid.length; y++) {
    const line = grid[y];
    for (let x = 0; x < line.length; x++) {
      if (line[x] === "^") {
        guardPos = [y, x];
      }
    }
  }
  return { grid, guardPos };
}

const dirVectors = [
  [-1, 0], // up
  [0, 1], // right
  [1, 0], // down
  [0, -1], // left
];

// for debugging
// deno-lint-ignore no-unused-vars
function drawGrid(
  grid: string[][],
  guardPos: [number, number],
  steps: Set<string>,
) {
  return grid.map((line, y) =>
    line.map((c, x) => {
      if (y === guardPos[0] && x === guardPos[1]) {
        return "G";
      }
      if (steps.has(`${y},${x}`)) {
        return "*";
      }
      return c;
    }).join("")
  ).join("\n");
}

export function part1(input: string) {
  const { grid, guardPos } = parseInput(input);

  const { steps } = runGrid(grid, guardPos);

  return steps.size.toString();
}

Deno.test("part 01 example", () => {
  assertEquals(part1(EXAMPLE_INPUT), "41");
});

Deno.test("part 01 input", () => {
  assertEquals(part1(INPUT), "5067");
});

function runGrid(
  grid: string[][],
  guardPos: [number, number],
  checkSteps = true,
) {
  let guardDir = 0;
  const steps = new Set<string>();
  const turns = new Set<string>();
  while (true) {
    const dir = dirVectors[guardDir];
    const np = [guardPos[0] + dir[0], guardPos[1] + dir[1]] as [
      number,
      number,
    ];

    if (
      np[0] < 0 ||
      np[1] < 0 ||
      np[0] >= grid.length ||
      np[1] >= grid[np[0]].length
    ) {
      break;
    }

    if (grid[np[0]][np[1]] === "#") {
      guardDir = (guardDir + 1) % 4;
      if (turns.has(`${np[0]},${np[1]}:${guardDir}`)) {
        throw new Error("Loop detected: " + np.join(","));
      }
      turns.add(`${np[0]},${np[1]}:${guardDir}`);
      continue;
    }
    guardPos = np;
    if (checkSteps) {
      steps.add(np.join(","));
    }
  }
  return { steps, turns };
}

export function part2(input: string) {
  const { grid, guardPos } = parseInput(input);

  const { steps } = runGrid(grid, guardPos);

  const loops: string[] = [];
  for (const step of steps) {
    const [y, x] = step.split(",").map(Number);
    grid[y][x] = "#";
    try {
      runGrid(grid, guardPos, false);
    } catch (e) {
      if (e instanceof Error && e.message.includes("Loop detected")) {
        loops.push(e.message.split(": ")[1]);
      } else {
        throw e;
      }
    } finally {
      grid[y][x] = ".";
    }
  }
  return loops.length.toString();
}

Deno.test("part 02 example", () => {
  assertEquals(part2(EXAMPLE_INPUT), "6");
});

Deno.test("part 02 input", () => {
  assertEquals(part2(INPUT), "1793");
});

const EXAMPLE_INPUT = `....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...`;

export const INPUT = Deno.readTextFileSync(
  `${import.meta.dirname}/../../../inputs/2024/day_06.txt`,
);
