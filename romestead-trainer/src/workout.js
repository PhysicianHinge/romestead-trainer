const exercises = {
  beginner: ['Push-ups', 'Squats', 'Plank'],
  intermediate: ['Pull-ups', 'Lunges', 'Dips', 'Burpees'],
  advanced: ['Pistol Squats', 'Muscle-ups', 'Handstand Push-ups']
};

function generateWorkout(level) {
  const list = exercises[level] || exercises.beginner;
  return list.sort(() => Math.random() - 0.5).slice(0, 3);
}

module.exports = { generateWorkout };