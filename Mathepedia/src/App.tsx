import { useLocation } from "react-router-dom";
import "./App.css";
import {TextView} from "./components/TextView"
import React from "react";


function useQuery() {
  const { search } = useLocation();

  return React.useMemo(() => new URLSearchParams(search), [search]);
}

function App() {
  let query = useQuery();
  
  let queryId = query.get("id");

  return (
    <div className="container">
      {
        queryId != null && <TextView id={parseInt(queryId!)}/>
      }
    </div>
  );
}

export default App;
