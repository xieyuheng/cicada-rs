const nb = import ("../wasm_modules/cicada_notebook");
import * as React from "react";
import * as ReactDOM from "react-dom";
import { useState } from "react";
import "./index.css";

let InputBuffer = (props: {
    focus_p: boolean
    value: string
    onChange: (event: any) => void
    onKeyDown: (event: any) => void
}) => {
    return <>
        <textarea
            className = "InputBuffer"
            autoFocus = {props.focus_p}
            value = {props.value}
            onChange = {props.onChange}
            onKeyDown = {props.onKeyDown}
        />
    </>
};

let OutputBuffer = (props: {
    value: string
}) => {
    return <>
        <textarea
            className = "OutputBuffer"
            value = {props.value}
        />
    </>
};

let Note = (props: {
    headline: string
    focus_p: boolean
    nb: any
    input: string
    output: string
    onChange: (event: any) => void
    onKeyDown: (event: any) => void
}) => {
    let out = <></>;
    if (props.output != "") {
        out = <OutputBuffer value = {props.output} />;
    }

    return <>
        <div className = "Note">
            <hr />
            <h3>{props.headline}</h3>
            {/* - focus_p: {props.focus_p.toString ()} */}
            <InputBuffer
                focus_p = {props.focus_p}
                value = {props.input}
                onChange = {props.onChange}
                onKeyDown = {props.onKeyDown}
            />
            {out}
            <hr />
        </div>
    </>
};

const WELCOME_MESSAGE = `\
// welcome to cicada-notebook ^-^/
// - short keys :
//   ctrl + enter -- run
//   alt  + enter -- run & open a note
`;

let Root = (props: {
    nb: any
}) => {
    let [noteList, setNoteList] = useState ([
        { input: WELCOME_MESSAGE, output: "" },
    ]);

    let [current, setCurrent] = useState (0);

    let onKeyDown = (index: number) => (event: any) => {
        if (event.key == "Enter" && event.ctrlKey) {
            let length = noteList.length;
            let list = noteList.slice (0, length);
            let input = list [index] .input;
            list [index] .output = props.nb.run (input);
            setNoteList (list);
        }
        if (event.key == "Enter" && event.altKey) {
            let length = noteList.length;
            setCurrent (index + 1);
            let list = noteList.slice (0, index + 1);
            let succ = noteList.slice (index + 1, length);
            let input = list [index] .input;
            list [index] .output = props.nb.run (input);
            list.push ({ input: "", output: "" });
            setNoteList (list.concat (succ));
        }
    };

    let onChange = (index: number) => (event: any) => {
        let length = noteList.length;
        let list = noteList.slice (0, length);
        list [index] .input = event.target.value;
        setNoteList (list);
    };

    let list = noteList.map ((io, index) => {
        return <>
            <Note
                headline = {"#" + (index + 1) .toString ()}
                focus_p = {index == current}
                nb = {props.nb}
                input = {io.input}
                output = {io.output}
                onChange = {onChange (index)}
                onKeyDown = {onKeyDown (index)}
            />
        </>
    });

    return <>
        {list}
    </>
};

nb.then (nb => {
    ReactDOM.render (
        <Root nb = {nb} />,
        document.getElementById ("root"));
});
