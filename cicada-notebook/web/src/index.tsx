const nb = import ("../wasm_modules/cicada_notebook");
import * as React from "react";
import * as ReactDOM from "react-dom";
import { useState } from "react";
import "./index.css";

let InputBuffer = (props: {
    onChange: (event: any) => void
}) => {
    return <textarea className = "Buffer"
                     onChange = {props.onChange}>
        // input
    </textarea>
};

let OutputBuffer = (props: {
    output: string
}) => {
    return <textarea className = "Buffer"
                     value = {props.output}>
    </textarea>
};

let RunButton = (props: {
    onClick: () => void
}) => {
    return <>
        <button className = "RunButton"
                onClick = {props.onClick}>
            run
        </button>
    </>
};

let NoteBook = (props: {
    nb: any
}) => {
    let [input, setInput] = useState ("");
    let [output, setOutput] = useState ("// output");

    let onClick = () => {
        setOutput (props.nb.run (input))
    };

    let onChange = (event: any) => {
        setInput (event.target.value);
    };

    return <>
        <p><RunButton onClick = {onClick} /></p>
        <InputBuffer onChange = {onChange} />
        <OutputBuffer output = {output} />
    </>
}

nb.then (nb => {
    ReactDOM.render (
        <NoteBook nb = {nb} />,
        document.getElementById ("root"));
});
