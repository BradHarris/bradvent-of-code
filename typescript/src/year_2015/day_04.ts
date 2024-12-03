import { crypto } from "jsr:@std/crypto/crypto";
import { assertEquals } from "jsr:@std/assert/equals";
import { encodeHex } from "jsr:@std/encoding/hex";

export function parseInput(input: string) {
  return input.split("\n");
}

export async function part1(input: string) {
  const encoder = new TextEncoder();
  for (let i = 0;; i++) {
    const data = encoder.encode(input + i.toString());
    const hash = await crypto.subtle.digest("MD5", data);

    const hex = encodeHex(hash);

    if (hex.slice(0, 5) === "00000") {
      return i.toString();
    }
  }
}

Deno.test("part 01 example", async () => {
  assertEquals(await part1("abcdef"), "609043");
});

Deno.test("part 01 input", async () => {
  assertEquals(await part1(INPUT), "282749");
});

export async function part2(input: string) {
  const encoder = new TextEncoder();
  for (let i = 0;; i++) {
    const data = encoder.encode(input + i.toString());
    const hash = await crypto.subtle.digest("MD5", data);

    const hex = encodeHex(hash);

    if (hex.slice(0, 6) === "000000") {
      return i.toString();
    }
  }
}

Deno.test("part 02 example", async () => {
  assertEquals(await part2("abcdef"), "6742839");
});

Deno.test("part 02 input", async () => {
  assertEquals(await part2(INPUT), "9962624");
});

export const INPUT = `yzbqklnj`;
