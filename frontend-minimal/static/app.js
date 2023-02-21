import init, { MDEmu as Megadrive } from './frontend_minimal.js';
import gamepad from './gamepad.js';

const canvas = document.querySelector('canvas');
const ctx = canvas.getContext('2d', { alpha: false });
const img = ctx.createImageData(320, 240);

(async () => {
    const instance = await init();

    const savedRom = window.localStorage.getItem('ROM');
    const rom = savedRom?.length ? Uint8Array.from(JSON.parse(savedRom)) : new Uint8Array(0);

    const emu = new Megadrive(rom);

    function draw() {
        const buffer = new Uint8ClampedArray(
            instance.memory.buffer, emu.screen(), 320 * 240 * 3
        );
        for (let i = 0; i < 320*240; i++) {
            const bufferIndex = i * 3;
            const imgIndex = i * 4;
            img.data[imgIndex+0] = buffer[bufferIndex+0];
            img.data[imgIndex+1] = buffer[bufferIndex+1];
            img.data[imgIndex+2] = buffer[bufferIndex+2];
            img.data[imgIndex+3] = 255;
        }

        ctx.putImageData(img, 0, 0);

        // ctx.putImageData(new ImageData(buffer, 320), 0, 0);
    }

    const frameCountEl = document.querySelector('.frameCount');
    const frames = [];
    (function loop() {
        requestAnimationFrame(loop);
        if (document.visibilityState !== 'hidden') {
            emu.gamepad_p1(gamepad());

            const frameCount = emu.render();
            if (frameCount > 0) draw();


            frames.push(frameCount);
            if (frames.length > 60) frames.shift();
            frameCountEl.textContent = `frames this frame: ${frameCount}\navg frames: ${(frames.reduce((a, b) => a + Number(b), 0) / frames.length).toFixed(2)}`;
        }
    })();

    // change ROM
    (document.querySelector('#file')).addEventListener(
        'change',
        (e) => {
            const reader = new FileReader();
            reader.readAsArrayBuffer(e.target.files[0]);
            reader.onloadend = () => {
                const rom = new Uint8Array(reader.result);
                emu.change_rom(rom);
                window.localStorage.setItem('ROM', JSON.stringify([...rom]));
            };
            e.preventDefault();
        },
    );
})();
