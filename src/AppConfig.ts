import * as config from '../config.json';

export interface AppConfig {
  victoria3Path: string;
  outputDir: string;
}

export function loadConfig(): AppConfig {
  return config;
}