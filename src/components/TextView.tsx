import {Component} from "react";
import "./textview_style.css"
import { invoke } from "@tauri-apps/api";

type TextViewState = {
    data: string
};

type TextViewProps = {
    id: number
};

export class TextView extends Component<TextViewProps, TextViewState> {
    state: TextViewState = {
        data: "",
    };

    async get_data() {
        let data:string = await invoke("get_reference_data", {id: this.props.id});
        this.setState({data: data});
    }

    componentDidMount(): void {
        this.get_data();
    }

    render(): React.ReactNode {
        return (
            <div className="theorem">
                <h1>Definition Konvergence</h1>
                <h2>Math in MathML notation</h2>
                <div dangerouslySetInnerHTML={{__html: this.state.data}}/>
            </div>
        )
    }
}
