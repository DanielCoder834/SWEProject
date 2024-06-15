// @author Adarsh Jayaram
import React, { Component } from "react";
import UserService from "../services/user.service";
import Spreadsheet from "./spreadsheets";
import TopMenu from "./topmenu";

type Props = {
  dimensions: { rows: number; columns: number };
  title: string;
  onCreateSpreadsheet: (rows: number, columns: number, title: string) => void;
};

type State = {
  content: string;
};

// Home component serves as the main page layout that includes top menu, spreadsheet, and potentially other components.
export default class Home extends Component<Props, State> {
  // Constructor: Initializes component state and binds methods.
  constructor(props: Props) {
    super(props);

    this.state = {
      content: ""
    };
  }

  // Render method: Outputs the structured UI of the Home component, integrating TopMenu and Spreadsheet components.
  render() {
    const { dimensions, title, onCreateSpreadsheet } = this.props;
    return (
      <div className="container">
        <TopMenu title={title} onCreateSpreadsheet={onCreateSpreadsheet} />
        <Spreadsheet dimensions={dimensions} />
        <header className="jumbotron">
          <h3>{this.state.content}</h3>
        </header>
      </div>
    );
  }
}

