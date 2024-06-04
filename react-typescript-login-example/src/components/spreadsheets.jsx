import React, { useState } from "react";

const TopMenu = () => {
  const [showFileMenu, setShowFileMenu] = useState(false);
  const [showUserMenu, setShowUserMenu] = useState(false);

  const handleFileClick = () => {
    setShowFileMenu(!showFileMenu);
  };

  const handleUserClick = () => {
    setShowUserMenu(!showUserMenu);
  };

  return (
    <div className="top-menu">
      <button onClick={handleFileClick}>File</button>
      {showFileMenu && (
        <ul>
          <li onClick={() => console.log("Create New Document")}>Create</li>
          <li onClick={() => console.log("Open Document")}>Open</li>
          <li onClick={() => console.log("Save Document")}>Save</li>
        </ul>
      )}

      <button onClick={handleUserClick}>Users</button>
      {showUserMenu && (
        <ul>
          <li>Bob</li>
          <li>Joe</li>
          <li>Sally</li>
        </ul>
      )}
    </div>
  );
};

export default TopMenu;