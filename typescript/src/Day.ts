export interface Day {
  part1: (input: string) => Promise<string> | string;
  part2: (input: string) => Promise<string> | string;
  INPUT: string;
}
