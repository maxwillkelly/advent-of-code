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

console.log(totalDistance);
