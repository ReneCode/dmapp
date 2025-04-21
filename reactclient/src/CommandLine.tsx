import React, { useState } from "react";

import "./CommandLine.css";

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
    <div className="commandline">
      <input
        className="command-input"
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={handleKeyPress}
        placeholder="Type a command and press Enter..."
      />
    </div>
  );
};

export default CommandLine;
