import React from "react";

const Input = ({ placeholder, value, onChange }) => {
  return (
    <input
      placeholder={placeholder}
      value={value}
      onChange={onChange}
    />
  );
};

export default Input;
