import { useEffect, useState } from "react";
import "./App.css";
import { getConfig } from "./config/mod";

function App() {
  const [val, setVal] = useState(<h1>Loading...</h1>);

  useEffect(() => {
    getConfig().then((value) => {
      setVal(<h1>{JSON.stringify(value)}</h1>);
    });
  });

  return <main className="container">{val}</main>;
}

export default App;
