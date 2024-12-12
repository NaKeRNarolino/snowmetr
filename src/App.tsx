import { useEffect, useState } from "react";
import "./App.css";
import { getConfig } from "./config/mod";
import { Async } from "react-async";
import { getSnowflakes, parseWidgetCode, widgetFrom } from "./snowflakes/mod";

function App() {
  return (
    <main className="container">
      <Async
        promiseFn={async () => {
          return await getConfig();
        }}
      >
        {(props) => {
          if (!props.data) return <h1>LOADING</h1>;

          const config = props.data!;

          const snowflakes_parsed = getSnowflakes(config);

          return <>{snowflakes_parsed}</>;
        }}
      </Async>
    </main>
  );
}

export default App;
