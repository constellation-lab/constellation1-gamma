import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import AnchorJS from "anchorjs";

const Trade = () => {
  const [option, setOption] = useState(null);
  const [quantity, setQuantity] = useState(1);
  const [price, setPrice] = useState(0);

  useEffect(() => {
    // Get the selected option from the state.
    setOption(selectedOption);
  }, [selectedOption]);

  const handleQuantityChange = (e) => {
    // Set the quantity.
    setQuantity(e.target.value);
  };

  const handlePriceChange = (e) => {
    // Set the price.
    setPrice(e.target.value);
  };

  const handleSubmit = (e) => {
    // Trade the option.
    // ...
    e.preventDefault();
  };

  return (
    <div>
      <h1>Trade</h1>
      <h2>Option</h2>
      <ul>
        <li>{option.underlyingAsset}</li>
        <li>{option.strikePrice}</li>
      </ul>
      <h2>Quantity</h2>
      <input
        type="number"
        min="1"
        max="100"
        value={quantity}
        onChange={handleQuantityChange}
      />
      <h2>Price</h2>
      <input
        type="number"
        min="0"
        max="10000"
        value={price}
        onChange={handlePriceChange}
      />
      <button onClick={handleSubmit}>Trade</button>
    </div>
  );
};

ReactDOM.render(<Trade />, document.getElementById("trade"));


/*
Trade.js

import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import AnchorJS from "anchorjs";

const Trade = () => {
  const [option, setOption] = useState(null);
  const [amount, setAmount] = useState(1);
  const [price, setPrice] = useState(0);

  useEffect(() => {
    // Get the selected option from the Cosm WASM contracts.
    AnchorJS.query("option", { id: option.id }).then((result) => {
      setOption(result.data);
      setPrice(result.data.strikePrice);
    });
  }, [option]);

  const handleAmountChange = (event) => {
    // Set the amount of options to trade.
    setAmount(parseInt(event.target.value, 10));
  };

  const handlePriceChange = (event) => {
    // Set the price of the option.
    setPrice(parseInt(event.target.value, 10));
  };

  const handleTrade = () => {
    // Trade the option.
    // ...
  };

  return (
    <div>
      <h1>OpynFinance</h1>
      <h2>Trade Option</h2>
      <ul>
        <li>
          <label>Option:</label>
          <input
            type="text"
            disabled
            value={option.underlyingAsset}
          />
        </li>
        <li>
          <label>Strike Price:</label>
          <input
            type="text"
            disabled
            value={price}
          />
        </li>
        <li>
          <label>Amount:</label>
          <input
            type="number"
            min="1"
            value={amount}
            onChange={handleAmountChange}
          />
        </li>
        <li>
          <button onClick={handleTrade}>Trade</button>
        </li>
      </ul>
    </div>
  );
};

ReactDOM.render(<Trade />, document.getElementById("trade"));



----


Trade.js

import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import AnchorJS from "anchorjs";

const Trade = () => {
  const [option, setOption] = useState(null);
  const [amount, setAmount] = useState(1);
  const [price, setPrice] = useState(0);
  const [error, setError] = useState(null);

  useEffect(() => {
    // Get the selected option from the Cosm WASM contracts.
    AnchorJS.query("option", { id: option.id }).then((result) => {
      setOption(result.data);
    });
  }, [option]);

  const handleAmountChange = (e) => {
    // Set the amount.
    setAmount(e.target.value);
  };

  const handlePriceChange = (e) => {
    // Set the price.
    setPrice(e.target.value);
  };

  const handleSubmit = (e) => {
    // Submit the trade.
    e.preventDefault();

    // Check if the amount is valid.
    if (amount <= 0) {
      setError("Invalid amount");
      return;
    }

    // Check if the price is valid.
    if (price <= 0) {
      setError("Invalid price");
      return;
    }

    // Trade the option.
    // ...
  };

  return (
    <div>
      <h1>Trade</h1>
      <input
        type="text"
        placeholder="Option ID"
        value={option.id}
        disabled
      />
      <input
        type="number"
        placeholder="Amount"
        value={amount}
        onChange={handleAmountChange}
      />
      <input
        type="number"
        placeholder="Price"
        value={price}
        onChange={handlePriceChange}
      />
      <button onClick={handleSubmit}>Trade</button>
      {error && <p>{error}</p>}
    </div>
  );
};

ReactDOM.render(<Trade />, document.getElementById("trade"));



*?
