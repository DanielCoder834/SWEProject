import { Component } from "react";

import UserService from "../services/user.service";
import Spreadsheets from './spreadsheets';
import TopMenu from './topmenu';

type Props = {};

type State = {
  content: string;
}

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
    return (
      <div className="container">
        <TopMenu />
        <Spreadsheets />
        <header className="jumbotron">
          <h3>{this.state.content}</h3>
        </header>
      </div>
    );
  }
}
