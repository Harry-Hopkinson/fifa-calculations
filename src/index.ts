import * as data from "../data/data.json";
import { Player } from "./player";
const prompt = require("prompt-sync")();

// this is the number of how
// many players average matches
// their total
let correct: number = 0;
let wrong: number = 0;
let diff: number = 0;
let strict: boolean = false;

strict = prompt("Strict mode? (y/n): ") === "y";

// loop through the data
for (let i = 0; i < data.length; i++) {
  // get the player
  const player: Player = data[i];

  // if the player is a defender
  if (player.POS === "CB" || player.POS === "LB" || player.POS === "RB") {
    let average: number =
      (Number(player.Defending) +
        Number(player.StandingTackle) +
        Number(player.SlidingTackle) +
        Number(player.Heading) +
        Number(player.Strength) +
        Number(player.Aggression)) /
      6;
    average = Math.round(average);

    if (average === Number(player.OVR)) {
      correct++;
      console.log(player.name + " is correct");
    } else {
      wrong++;
      diff += Math.abs(average - Number(player.OVR));
      console.log(diff);
    }
    diff = 0;
  } else if (
    player.POS === "CM" ||
    player.POS === "CDM" ||
    player.POS === "CAM"
  ) {
    let average: number =
      (Number(player.Passing) +
        Number(player.Dribbling) +
        Number(player.Vision) +
        Number(player.Crossing) +
        Number(player.ShortPassing) +
        Number(player.LongPassing)) /
      6;
    average = Math.round(average);

    // if the average equals the ovr

    if (average === Number(player.OVR)) {
      correct++;
      console.log(player.name + " is correct");
    } else {
      wrong++;
      diff += Math.abs(average - Number(player.OVR));
      console.log(diff);
    }
    diff = 0;
  }
}

console.log("Correct: " + correct + " Wrong: " + wrong);
