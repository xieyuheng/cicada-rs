import * as React from "react";
import * as ReactDOM from "react-dom";
import { useState } from "react";
import "./index.css";

let Square = (props: {
    value: string
    onClick: () => void
}) => {
    return <>
        <button className = "square" onClick = {props.onClick}>
            {props.value}
        </button>
    </>
};

let Board = () => {
    let [squares, setSquares] = useState ([
        "", "", "",
        "", "", "",
        "", "", "",
    ]);

    let [next, setNext] = useState ("X");

    let handleClick = (i: number) => {
        let s = squares.slice ();
        if (calculateWinner (s) || squares[i]) {
            return;
        }
        s [i] = next;
        setSquares (s);
        if (next == "X") {
            setNext ("O");
        } else {
            setNext ("X");
        }
    };

    let renderSquare = (i: number) => {
        return (
            <Square
                value = {squares [i]}
                onClick = {() => handleClick (i)}
            />
        );
    };

    let status;
    let winner = calculateWinner (squares);
    if (winner) {
        status = "Winner: " + winner;
    } else {
        status = "Next player: " + next;
    }

    return <>
        <div>
            <div className="status">{status}</div>
            <div className="board-row">
                {renderSquare (0)}
                {renderSquare (1)}
                {renderSquare (2)}
            </div>
            <div className="board-row">
                {renderSquare (3)}
                {renderSquare (4)}
                {renderSquare (5)}
            </div>
            <div className="board-row">
                {renderSquare (6)}
                {renderSquare (7)}
                {renderSquare (8)}
            </div>
        </div>
    </>
};

let Game = () => {
    return <>
        <div className="game">
            <div className="game-board">
                <Board />
            </div>
            <div className="game-info">
                <div>{/* status */}</div>
                <ol>{/* TODO */}</ol>
            </div>
        </div>
    </>
};

let calculateWinner = (
    squares: string[]
): null | string => {
    const lines = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for (let i = 0; i < lines.length; i++) {
        const [a, b, c] = lines[i];
        if (squares[a] &&
            squares[a] === squares[b] &&
            squares[a] === squares[c]
        ) {
            return squares[a];
        }
    }
    return null;
};

let Root = () => <>
    <Game />
</>;

ReactDOM.render (<Root />, document.getElementById ("root"));
