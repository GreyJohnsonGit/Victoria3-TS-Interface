import * as FileSystem from 'fs';

export interface AppConfig {
  victoria3Path: string;
  outputDir: string;
}

export function loadConfig(): AppConfig {
  return JSON.parse(FileSystem.readFileSync('config.json', 'utf8'));
}