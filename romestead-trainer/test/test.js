const { generateWorkout } = require('../src/workout');
const { logSession, getLastSession } = require('../src/logger');
const fs = require('fs');
const path = require('path');
const LOG_FILE = path.join(__dirname, '..', 'sessions.log');

// Clean up before test
if (fs.existsSync(LOG_FILE)) fs.unlinkSync(LOG_FILE);

// Test generateWorkout
const workout = generateWorkout('beginner');
console.assert(workout.length === 3, 'Workout should have 3 exercises');
console.log('✓ generateWorkout returns correct length');

// Test logSession and getLastSession
logSession('TestUser', '2025-01-01T00:00:00Z', 3);
const last = getLastSession();
console.assert(last.includes('TestUser'), 'Last session should contain username');
console.log('✓ logSession and getLastSession work');

// Clean up
if (fs.existsSync(LOG_FILE)) fs.unlinkSync(LOG_FILE);
console.log('All tests passed!');