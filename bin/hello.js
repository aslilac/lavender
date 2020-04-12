const chalk = require("chalk");

console.log(chalk`
Hello! 👋

  {magenta →} Make sure you've installed all the dependencies listed in the README!

  {cyan →} Run {bold yarn} {bold.hex('#9084e6') dev} to build a version for debugging and development {bold.cyan localhost:1234}
  {cyan →} Run {bold yarn} {bold.hex('#9084e6') release} to build an optimized version for playing games {bold.cyan localhost:1234}
`);
