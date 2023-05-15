import React from "react";

const Table = ({ columns, data }) => {
  return (
    <table>
      <thead>
        {columns.map((column) => (
          <th key={column.id}>{column.label}</th>
        ))}
      </thead>
      <tbody>
        {data.map((row) => (
          <tr key={row.id}>
            {row.map((cell) => (
              <td key={cell.id}>{cell}</td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default Table;
