// Advent of Code: 2024, Day 1
// Problem: https://adventofcode.com/2024/day/1
// Solution:
//   Part 1: Read the input file, split the lines into two lists, sort the lists,
//          calculate the distance between the elements of the lists, and sum the distances.
//   Part 2: Calculate the similarity between the elements of the lists and sum the similarities.

import * as fs from "fs";

const textFile = fs.readFileSync("./input/input.txt", "utf8");
const lines = textFile.split("\n");

type Lists = {
  left: number[];
  right: number[];
};

const lists = lines.reduce<Lists>(
  (acc, line) => {
    const [leftListString, rightListString] = line.split("   ");

    if (leftListString === undefined || rightListString === undefined) {
      return acc;
    }

    return {
      left: [...acc.left, parseInt(leftListString)],
      right: [...acc.right, parseInt(rightListString)],
    };
  },
  {
    left: [],
    right: [],
  }
);

const leftListSorted = lists.left.sort((a, b) => a - b);
const rightListSorted = lists.right.sort((a, b) => a - b);

const distances = leftListSorted.map((left, index) => {
  const right = rightListSorted[index];
  return Math.abs(left - right);
});

const totalDistance = distances.reduce((acc, distance) => {
  return acc + distance;
}, 0);

// Output: 2196996
console.log("Distance:", totalDistance);

const similarities = lists.left.map((left, index) => {
  const appearsInRight = lists.right.filter((right) => right === left).length;
  return left * appearsInRight;
});

const totalSimilarity = similarities.reduce((acc, similarity) => {
  return acc + similarity;
}, 0);

// Output: 23655822
console.log("Similiarity:", totalSimilarity);
