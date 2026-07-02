const chalk = require('chalk');
const { generateWorkout } = require('./workout');
const { logSession } = require('./logger');

const user = { name: 'Romestead', level: 'intermediate' };

console.log(chalk.green(`🏋️ Welcome to Romestead Trainer, ${user.name}!`));
const workout = generateWorkout(user.level);
console.log(chalk.cyan('Today\'s workout:'));
workout.forEach((ex, i) => console.log(chalk.yellow(`${i + 1}. ${ex}`)));

logSession(user.name, new Date().toISOString(), workout.length);
console.log(chalk.magenta('Session logged. Stay strong!'));