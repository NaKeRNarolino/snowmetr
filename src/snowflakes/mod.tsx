import * as tauri from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api/core";
import { appDataDir, documentDir } from "@tauri-apps/api/path";
import parse from "html-react-parser";
import { SnowmetrConfig } from "../config/mod";
import { Async } from "react-async";
import { importFromString, requireFromString } from "module-from-string";
import StringToReactComponent from "string-to-react-component";

export interface Vec2Str {
  x: string;
  y: string;
}

export interface Snowflake {
  label: string;
  path: string;
  position: Vec2Str;
  scale: number;
  custom_props: Map<string, string | number>;
}

export async function parseWidgetCode(snowflake: Snowflake): Promise<string> {
  let path = snowflake.path;
  let html_doc = (await invoke("read_file", { file: path })) as string;

  return html_doc;
}

export function widgetFrom(data: string, snowflake: Snowflake): JSX.Element {
  return (
    <div
      style={{
        position: "absolute",
        top: `${snowflake.position.y}`,
        left: `${snowflake.position.x}`,
      }}
    >
      <StringToReactComponent data={snowflake.custom_props}>
        {data}
      </StringToReactComponent>
    </div>
  );
}

export function getSnowflakes(config: SnowmetrConfig): JSX.Element[] {
  const children: JSX.Element[] = [];

  config.snowflakes.forEach((snowflake) => {
    children.push(
      <Async
        promiseFn={async () => {
          return await parseWidgetCode(snowflake);
        }}
      >
        {({ data }) => {
          if (!data) return <h1>LOADING</h1>;
          return widgetFrom(data, snowflake);
        }}
      </Async>
    );
  });

  return children;
}
