const nb = import ("../wasm_modules/cicada_notebook");
import * as React from "react";
import * as ReactDOM from "react-dom";
import { useState } from "react";
import "./index.css";

let InputBuffer = (props: {
    onChange: (event: any) => void
    onKeyDown: (event: any) => void
}) => {
    return <>
        <textarea className = "Buffer"
                  onChange = {props.onChange}
                  onKeyDown = {props.onKeyDown}
        >
            {INPUT_BUFFER_MESSAGE}
        </textarea>
    </>
};

let OutputBuffer = (props: {
    output: string
}) => {
    return <>
        <textarea className = "Buffer"
                  value = {props.output}>
        </textarea>
    </>
};

let RunButton = (props: {
    onClick: () => void
}) => {
    return <>
        <button className = "RunButton"
                onClick = {props.onClick}>
            RUN
        </button>
    </>
};

const INPUT_BUFFER_MESSAGE = `\
// - INPUT-BUFFER -
//   - CTRL + RETURN -- run
`;

// const INPUT_BUFFER_MESSAGE = `\
// // - INPUT-BUFFER -
// //   - CTRL + RETURN -- run
// //   - ALT  + RETURN -- run and new note
// `;

const OUTPUT_BUFFER_MESSAGE = `\
// - OUTPUT-BUFFER -
`;

let Note = (props: {
    nb: any
}) => {
    let [input, setInput] = useState (INPUT_BUFFER_MESSAGE);
    let [output, setOutput] = useState (OUTPUT_BUFFER_MESSAGE);

    let onKeyDown = (event: any) => {
        if (event.ctrlKey && event.key == "Enter") {
            setOutput (props.nb.run (input))
        }
        if (event.altKey && event.key == "Enter") {
            setOutput (props.nb.run (input))
        }
    };

    let onChange = (event: any) => {
        setInput (event.target.value);
    };

    let onClick = () => {
        setOutput (props.nb.run (input))
    };

    return <>
        <div className = "Note">
            <hr />
            <p><InputBuffer onChange = {onChange}
                            onKeyDown = {onKeyDown} /></p>
            <p><OutputBuffer output = {output} /></p>
            <hr />
        </div>
    </>
};

nb.then (nb => {
    ReactDOM.render (
        <Note nb = {nb} />,
        document.getElementById ("root"));
});
