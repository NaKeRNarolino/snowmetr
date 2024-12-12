import { invoke } from "@tauri-apps/api/core";
import { Snowflake } from "../snowflakes/mod";

export interface SnowmetrConfig {
  snowflakes: Snowflake[];
}

export async function getConfig(): Promise<SnowmetrConfig> {
  console.log("path ^");
  const data = (await invoke("get_config")) as string;
  // const path = "";
  const json = JSON.parse(data) as SnowmetrConfig;

  return json;
}
