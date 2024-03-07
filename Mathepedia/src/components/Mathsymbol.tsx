import {Component, ReactNode} from "react";

type MathSymbolProps = {
    text: string;
};

export class MathSymbol extends Component<MathSymbolProps> {
    constructor(props: MathSymbolProps) {
        super(props);
    }

    render(): ReactNode {
        return <div>{this.props.text}</div>
    }
}

export class Forall {
    render(): ReactNode {
        return <MathSymbol text="∀" />
    }
}

export class Exists {
    render(): ReactNode {
        return <MathSymbol text="∃" />
    }
}

export class GreaterThan {
    render(): ReactNode {
        return <MathSymbol text=">" />
    }
}
