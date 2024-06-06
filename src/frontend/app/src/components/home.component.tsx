import { Component } from "react";
import UserService from "../services/user.service";
import Spreadsheet from './spreadsheets';
import TopMenu from './topmenu';

type Props = {
  dimensions: { rows: number; columns: number };
  onCreateSpreadsheet: (rows: number, columns: number, title: string) => void;
};

type State = {
  content: string;
  title: string;
};

export default class Home extends Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {
      content: "",
      title: "Untitled"
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

  handleCreateSpreadsheet = (rows: number, columns: number, title: string) => {
    this.setState({
      title: title,
    });
    this.props.onCreateSpreadsheet(rows, columns, title);
  };

  render() {
    const { dimensions } = this.props;
    const { title } = this.state;

    return (
      <div className="container">
        <TopMenu onCreateSpreadsheet={this.handleCreateSpreadsheet} title={title} />
        <Spreadsheet dimensions={dimensions} />
        <header className="jumbotron">
          <h3>{this.state.content}</h3>
        </header>
      </div>
    );
  }
}
