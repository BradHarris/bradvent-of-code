export interface Day {
  parseInput: (input: string) => Promise<any>;
  part1: (input: string) => Promise<string>;
  part2: (input: string) => Promise<string>;
  INPUT: string;
}
