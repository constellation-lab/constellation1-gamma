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
