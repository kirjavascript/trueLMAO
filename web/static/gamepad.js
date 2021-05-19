const controls = new Set([]);

window.addEventListener('blur', () => { controls.clear(); });
window.addEventListener('focus', () => { controls.clear(); });

const keymap = {};

[
    [1 << 7, ['Enter', 'Shift']], // start
    [1 << 6, ['z', 'Z', ',', '<']], // A
    [1 << 5, ['x', 'X', '.', '>', ' ']], // B
    [1 << 4, ['c', 'C', '/', '?']], // C
    [1 << 3, ['ArrowRight', 'd', 'D']], // R
    [1 << 2, ['ArrowLeft', 'a', 'A']], // L
    [1 << 1, ['ArrowDown', 's', 'S']], // D
    [1, ['ArrowUp', 'w', 'W']], // U
].forEach(([value, keys]) => {
    keys.forEach(key => { keymap[key] = value; });
});

const html = document.documentElement;

html.addEventListener('keydown', e => {
    if (e.key in keymap) {
        controls.add(e.key);
        e.preventDefault();
    }
});
html.addEventListener('keyup', e => {
    if (e.key in keymap) {
        controls.delete(e.key);
        e.preventDefault();
    }
});

export default function() {
    return [...controls].reduce((a, c) => a | keymap[c], 0);
}
