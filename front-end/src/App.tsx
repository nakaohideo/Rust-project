import React from "react";
import "./App.css";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import AuthSide from "./page/AuthSide";
import Landing from './page/Landing'

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" Component={AuthSide}></Route>
        <Route path="/landing" Component={Landing}></Route>
      </Routes>
    </Router>
  );
}

export default App;
