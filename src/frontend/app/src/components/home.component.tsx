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

export default class Home extends Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {
      content: ""
    };
  }

  componentDidMount() {
    UserService.getPublicContent().then(
      response => {
        this.setState({
          content: response.data
        });
      },
      error => {
        this.setState({
          content:
            (error.response && error.response.data) ||
            error.message ||
            error.toString()
        });
      }
    );
  }

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
