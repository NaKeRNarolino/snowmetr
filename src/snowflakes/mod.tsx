import * as tauri from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api/core";
import { appDataDir, documentDir } from "@tauri-apps/api/path";

export interface Vec2 {
  x: number;
  y: number;
}

export interface Snowflake {
  label: string;
  path: string;
  position: Vec2;
  scale: number;
}
