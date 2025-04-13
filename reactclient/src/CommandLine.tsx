import React, { useState } from "react";

interface CommandLineProps {
  onChanged: (input: string) => void;
}

const CommandLine: React.FC<CommandLineProps> = ({ onChanged }) => {
  const [input, setInput] = useState("");

  const handleKeyPress = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Enter") {
      onChanged(input);
      setInput(""); // Clear the input field after calling the handler
    }
  };

  return (
    <input
      style={{ width: "100%", padding: "10px", fontSize: "16px" }}
      type="text"
      value={input}
      onChange={(e) => setInput(e.target.value)}
      onKeyDown={handleKeyPress}
      placeholder="Type a command and press Enter..."
    />
  );
};

export default CommandLine;
