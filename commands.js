import 'dotenv/config';
import { InstallGlobalCommands } from './utils.js';

// Simple test command
const TEST_COMMAND = {
  name: 'test',
  description: 'Basic command',
  type: 1,
  integration_types: [0, 1],
  contexts: [0, 1, 2],
};



// Echo back the second argument baby
const ECHO_COMMAND = {
  name: 'echo',
  description: 'Return back what you send!',
  options: [
    {
      type: 3,
      name: 'text',
      description: 'What do you want to echo back?',
      required: true,
    },
  ],
  type: 1,
  integration_types: [0, 1],
  contexts: [0, 1, 2],
};


const ALL_COMMANDS = [TEST_COMMAND, ECHO_COMMAND];

InstallGlobalCommands(process.env.APP_ID, ALL_COMMANDS);
