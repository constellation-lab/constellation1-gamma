
import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import AnchorJS from "anchorjs";

const Home = () => {
  const [options, setOptions] = useState([]);
  const [selectedOption, setSelectedOption] = useState(null);

  useEffect(() => {
    // Get the list of options from the Cosm WASM contracts.
    AnchorJS.query("option").then((result) => {
      setOptions(result.data);
    });
  }, []);

  const handleOptionSelect = (option) => {
    // Set the selected option.
    setSelectedOption(option);
  };

  const handleTrade = () => {
    // Trade the selected option.
    // ...
  };

  return (
    <div>
      <h1>OpynFinance</h1>
      <ul>
        {options.map((option) => (
          <li key={option.id}>
            <input
              type="checkbox"
              checked={option === selectedOption}
              onChange={() => handleOptionSelect(option)}
            />
            {option.underlyingAsset} - {option.strikePrice}
          </li>
        ))}
      </ul>
      <button onClick={handleTrade}>Trade</button>
    </div>
  );
};

ReactDOM.render(<Home />, document.getElementById("home"));


/*
Home.js

import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import AnchorJS from "anchorjs";

const Home = () => {
  const [options, setOptions] = useState([]);
  const [selectedOption, setSelectedOption] = useState(null);

  useEffect(() => {
    // Get the list of options from the Cosm WASM contracts.
    AnchorJS.query("option").then((result) => {
      setOptions(result.data);
    });
  }, []);

  const handleOptionSelect = (option) => {
    // Set the selected option.
    setSelectedOption(option);
  };

  const handleTrade = () => {
    // Trade the selected option.
    // ...
  };

  return (
    <div>
      <h1>OpynFinance</h1>
      <ul>
        {options.map((option) => (
          <li key={option.id}>
            <input
              type="checkbox"
              checked={option === selectedOption}
              onChange={() => handleOptionSelect(option)}
            />
            {option.underlyingAsset} - {option.strikePrice}
          </li>
        ))}
      </ul>
      <button onClick={handleTrade}>Trade</button>
    </div>
  );
};

ReactDOM.render(<Home />, document.getElementById("home"));



Home.js

import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import AnchorJS from "anchorjs";

const Home = () => {
  const [options, setOptions] = useState([]);
  const [selectedOption, setSelectedOption] = useState(null);

  useEffect(() => {
    // Get the list of options from the Cosm WASM contracts.
    AnchorJS.query("option").then((result) => {
      setOptions(result.data);
    });
  }, []);

  const handleOptionSelect = (option) => {
    // Set the selected option.
    setSelectedOption(option);
  };

  const handleTrade = () => {
    // Trade the selected option.
    // ...
  };

  return (
    <div>
      <h1>OpynFinance</h1>
      <ul>
        {options.map((option) => (
          <li key={option.id}>
            <input
              type="checkbox"
              checked={option === selectedOption}
              onChange={() => handleOptionSelect(option)}
            />
            {option.underlyingAsset} - {option.strikePrice}
          </li>
        ))}
      </ul>
      <button onClick={handleTrade}>Trade</button>
    </div>
  );
};

ReactDOM.render(<Home />, document.getElementById("home"));



*/
