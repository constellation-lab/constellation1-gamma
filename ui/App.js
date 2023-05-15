/*This code creates a simple web application that allows users to view a list of options and trade them. 
The application is connected to the OpynFinance Cosm WASM contracts using the AnchorJS library.*/

import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import { Anchor } from "anchor-js";

const App = () => {
  const [options, setOptions] = useState([]);
  const [selectedOption, setSelectedOption] = useState(null);

  useEffect(() => {
    // Get the list of options from the blockchain.
    const anchor = new Anchor("https://anchor.projectserum.com/opyn-finance");
    anchor.query("options", (err, result) => {
      if (err) {
        console.log(err);
      } else {
        setOptions(result.options);
      }
    });
  }, []);

  const handleSelectOption = (option) => {
    setSelectedOption(option);
  };

  const handleCreateOption = () => {
    // Create a new option.
    const anchor = new Anchor("https://anchor.projectserum.com/opyn-finance");
    anchor.send("createOption", {
      underlyingAsset: "uatom",
      strikePrice: 100,
      expirationDate: "2023-05-15T00:00:00Z",
      amount: 1,
    }, (err, result) => {
      if (err) {
        console.log(err);
      } else {
        console.log(result);
      }
    });
  };

  const handleExerciseOption = () => {
    // Exercise an option.
    const anchor = new Anchor("https://anchor.projectserum.com/opyn-finance");
    anchor.send("exerciseOption", {
      optionId: selectedOption.id,
    }, (err, result) => {
      if (err) {
        console.log(err);
      } else {
        console.log(result);
      }
    });
  };

  return (
    <div>
      <h1>Opyn Finance</h1>
      <ul>
        {options.map((option) => (
          <li key={option.id}>
            <a href="#" onClick={() => handleSelectOption(option)}>
              {option.underlyingAsset} {option.strikePrice} {option.expirationDate}
            </a>
          </li>
        ))}
      </ul>
      <h2>Selected Option</h2>
      {selectedOption ? (
        <ul>
          <li>Underlying Asset: {selectedOption.underlyingAsset}</li>
          <li>Strike Price: {selectedOption.strikePrice}</li>
          <li>Expiration Date: {selectedOption.expirationDate}</li>
          <li>Amount: {selectedOption.amount}</li>
        </ul>
      ) : (
        <p>No option selected</p>
      )}
      <h2>Create Option</h2>
      <form onSubmit={handleCreateOption}>
        <input
          type="text"
          placeholder="Underlying Asset"
          name="underlyingAsset"
        />
        <input
          type="text"
          placeholder="Strike Price"
          name="strikePrice"
        />
        <input
          type="date"
          placeholder="Expiration Date"
          name="expirationDate"
        />
        <input
          type="number"
          placeholder="Amount"
          name="amount"
        />
        <input type="submit" value="Create Option" />
      </form>
      <h2>Exercise Option</h2>
      {selectedOption ? (
        <form onSubmit={handleExerciseOption}>
          <input type="hidden" name="optionId" value={selectedOption.id} />
          <input type="submit" value="Exercise Option" />
        </form>
      ) : (
        <p>No option selected</p>
      )}
    </div>
  );
};

const rootElement = document.getElementById("root");
ReactDOM.render(<App />, rootElement);


/*
import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import AnchorJS from "anchor-js";

const App = () => {
  const [options, setOptions] = useState([]);
  const [selectedOption, setSelectedOption] = useState(null);

  useEffect(() => {
    // Get options from the OpynFinance Cosm WASM contracts
    AnchorJS.get("/options", (err, options) => {
      if (err) {
        console.log(err);
      } else {
        setOptions(options);
      }
    });
  }, []);

  const handleOptionSelect = (option) => {
    setSelectedOption(option);
  };

  const handleOptionTrade = () => {
    // Trade the selected option
    AnchorJS.post("/trade", {
      optionId: selectedOption.id,
      amount: 1,
    });
  };

  return (
    <div>
      <h1>OpynFinance</h1>
      <ul>
        {options.map((option) => (
          <li key={option.id}>
            <button onClick={() => handleOptionSelect(option)}>
              {option.symbol}
            </button>
          </li>
        ))}
      </ul>
      {selectedOption && (
        <div>
          <h2>{selectedOption.symbol}</h2>
          <button onClick={() => handleOptionTrade()}>Trade</button>
        </div>
      )}
    </div>
  );
};

const rootElement = document.getElementById("root");
ReactDOM.render(<App />, rootElement);



opt 3

import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import AnchorJS from "anchorjs";

const App = () => {
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

ReactDOM.render(<App />, document.getElementById("app"));




*/
