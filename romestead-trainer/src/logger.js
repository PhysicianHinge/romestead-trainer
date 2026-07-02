const fs = require('fs');
const path = require('path');

const LOG_FILE = path.join(__dirname, '..', 'sessions.log');

function logSession(name, date, exerciseCount) {
  const entry = `[${date}] ${name} completed ${exerciseCount} exercises\n`;
  fs.appendFileSync(LOG_FILE, entry, 'utf-8');
}

function getLastSession() {
  if (!fs.existsSync(LOG_FILE)) return null;
  const lines = fs.readFileSync(LOG_FILE, 'utf-8').trim().split('\n');
  return lines[lines.length - 1] || null;
}

module.exports = { logSession, getLastSession };