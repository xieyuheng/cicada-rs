const nb = import ("./cicada_notebook");

function get_element (id) {
    return document.getElementById (id);
}

let input_buffer  = get_element ("input_buffer");
let output_buffer = get_element ("output_buffer");

input_buffer.focus ();

function bindkeys (nb) {
    input_buffer.onkeydown = (event) => {
        if (event.key === "F2") {
            let input = input_buffer.value;
            output_buffer.value = nb.wis (input);
            input_buffer.focus ();
        }
    };
}

nb.then (nb => {
    bindkeys (nb);
});
