import React, { Fragment, useEffect } from 'react';
import { render } from 'react-dom';
import { default as init, get_string } from '#wasm'; // eslint-disable-line

function App(props) {
    return (
        <Fragment>
            trueLMAO
        </Fragment>
    );
}

if (typeof WebAssembly !== 'object') {
    document.body.innerHTML = 'this website requires WebAssembly - update your browser';
} else {
    delete WebAssembly.instantiateStreaming;
    init('/emu.wasm')
        .then(obj => {
            console.log(get_string());

            render((
                <App />
            ), document.body.appendChild(document.createElement('div')));
        })
        .catch(print.error);
}